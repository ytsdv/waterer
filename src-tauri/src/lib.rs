use std::sync::Mutex as SyncMutex;

use serde::Serialize;
use tauri::{Manager, RunEvent};
use tokio::{sync::Mutex, time::Duration};
mod db;
use db::init_db;
mod notification;
mod settings;
mod sip;
mod tray;
mod update;

use crate::{
    db::DatabaseState,
    notification::notify_sip,
    settings::AppSettings,
    sip::{get_sips, SipState},
    tray::{create_tray, update_timer_menu_item},
    update::{update, check_for_updates, install_update},
};

#[derive(Serialize, Clone)]
struct AppState {
    timer_started: bool,
    last_update_check: Option<std::time::SystemTime>,
}

impl AppState {
    fn new() -> Self {
        Self {
            timer_started: false,
            last_update_check: None,
        }
    }

    fn start_timer(&mut self) {
        if !self.timer_started {
            self.timer_started = true;
        }
    }

    fn stop_timer(&mut self) {
        if self.timer_started {
            self.timer_started = false;
        }
    }

    fn update_last_update_check(&mut self) {
        self.last_update_check = Some(std::time::SystemTime::now());
    }

    fn should_check_for_updates(&self, interval_hours: u64) -> bool {
        match self.last_update_check {
            Some(last_check) => {
                let elapsed = last_check.elapsed().unwrap_or(Duration::from_secs(0));
                elapsed >= Duration::from_secs(interval_hours * 3600)
            }
            None => true, // Never checked before
        }
    }
}

#[tauri::command]
async fn toggle_timer(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, SyncMutex<AppState>>,
    _sip_state: tauri::State<'_, Mutex<SipState>>,
) -> Result<(), String> {
    use tauri::Emitter;

    println!("toggle timer");
    let _timer_started = {
        let mut app_state = app_state.lock().map_err(|e| e.to_string())?;

        if app_state.timer_started {
            app_state.stop_timer();
        } else {
            app_state.start_timer();
        }

        // Update the tray menu item
        if let Err(e) = update_timer_menu_item(&app, app_state.timer_started) {
            eprintln!("Failed to update timer menu item: {}", e);
        }

        app.emit("update-app-state", app_state.clone())
            .map_err(|e| e.to_string())?;

        app_state.timer_started
    };
    Ok(())
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn get_app_state(
    app_state: tauri::State<'_, SyncMutex<AppState>>,
) -> Result<AppState, String> {
    match app_state.lock() {
        Ok(app_state) => Ok(app_state.clone()),
        Err(e) => Err(format!("Failed to fetch sips: {}", e)),
    }
}

#[tauri::command]
async fn take_sip(
    db_state: tauri::State<'_, DatabaseState>,
    sip_state: tauri::State<'_, Mutex<SipState>>,
) -> Result<SipState, String> {
    let pool = &db_state.0;

    let mut locked_sip_state = sip_state.lock().await;

    match locked_sip_state.take_sip(50, pool).await {
        Ok(new_state) => {
            let state_to_return = new_state.clone();
            *locked_sip_state = new_state;
            Ok(state_to_return)
        }
        Err(e) => Err(e.to_string()),
    }
}

// Command to get update check interval from frontend settings
#[tauri::command]
async fn get_update_check_interval() -> Result<u64, String> {
    // Since settings are currently stored in localStorage on frontend,
    // we'll return a default value and let the frontend manage this
    // TODO: When backend settings are implemented, get from there
    Ok(24) // Default 24 hours
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_sips,
            toggle_timer,
            take_sip,
            get_app_state,
            check_for_updates,
            install_update,
            get_update_check_interval
        ])
        .setup(|app| {
            let app_settings = AppSettings::load();
            app.manage(app_settings);

            init_db();

            let (db_state, sip_state) = tauri::async_runtime::block_on(async move {
                let database = db::Database::new()
                    .await
                    .expect("failed to initialize database");

                // Clone the pool before moving it
                let cloned_pool = database.pool.clone();

                let sip_state = SipState::default().read_from_db(&cloned_pool).await;

                (db::DatabaseState(database.pool), sip_state)
            });

            let app_state = AppState::new();
            app.manage(SyncMutex::new(app_state));

            // Store database pool in app state
            app.manage(db_state);
            app.manage(Mutex::new(sip_state));

            let db_state = app.state::<DatabaseState>();

            let cloned_pool = db_state.0.clone();

            // Clone the app handle so it can be moved into the spawned task
            let app_handle = app.handle().clone();

            // Spawn sip timer task
            let _ = tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));

                loop {
                    let timer_started = {
                        let app_state = app_handle.state::<SyncMutex<AppState>>();
                        let app_state = app_state.lock().unwrap();
                        app_state.timer_started
                    };

                    interval.tick().await;

                    let sip_state = app_handle.state::<Mutex<SipState>>();
                    let mut locked_sip_state = sip_state.lock().await;

                    if !timer_started {
                        continue;
                    }

                    if locked_sip_state.check_if_sip_is_due() {
                        if !locked_sip_state.notified_user {
                            match notify_sip(&app_handle) {
                                Ok(_) => {
                                    locked_sip_state.set_notified_user(true, &cloned_pool).await;
                                }
                                Err(e) => {
                                    eprintln!("Failed to notify user: {}", e);
                                }
                            }
                        }
                    }
                }
            });

            // Spawn update check task
            let app_handle_for_updates = app.handle().clone();
            let _ = tauri::async_runtime::spawn(async move {
                let mut update_check_interval = tokio::time::interval(Duration::from_secs(300)); // Check every 5 minutes if updates are due

                loop {
                    update_check_interval.tick().await;
                    
                    // Check if we should check for updates
                    let should_check = {
                        let app_state = app_handle_for_updates.state::<SyncMutex<AppState>>();
                        let app_state = app_state.lock().unwrap();
                        
                        // Default to 24 hours if no setting is available
                        let update_interval_hours = 24; // TODO: Get this from settings when implemented
                        app_state.should_check_for_updates(update_interval_hours)
                    };

                    if should_check {
                        // Update the last check time
                        {
                            let app_state = app_handle_for_updates.state::<SyncMutex<AppState>>();
                            let mut app_state = app_state.lock().unwrap();
                            app_state.update_last_update_check();
                        }

                        // Perform update check
                        match check_for_updates(app_handle_for_updates.clone()).await {
                            Ok(update_info) => {
                                if update_info.available {
                                    use tauri::Emitter;
                                    let _ = app_handle_for_updates.emit("update-available", update_info);
                                }
                            }
                            Err(e) => {
                                eprintln!("Periodic update check failed: {}", e);
                            }
                        }
                    }
                }
            });

            // Initial update check on startup
            let app_handle_initial = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // Wait a bit for app to fully initialize
                tokio::time::sleep(Duration::from_secs(3)).await;
                
                match check_for_updates(app_handle_initial.clone()).await {
                    Ok(update_info) => {
                        if update_info.available {
                            use tauri::Emitter;
                            let _ = app_handle_initial.emit("update-available", update_info);
                        }
                    }
                    Err(e) => {
                        eprintln!("Initial update check failed: {}", e);
                    }
                }
                
                // Mark that we've done an initial check
                {
                    let app_state = app_handle_initial.state::<SyncMutex<AppState>>();
                    let mut app_state = app_state.lock().unwrap();
                    app_state.update_last_update_check();
                }
            });

            let app_handle = app.handle().clone();

            create_tray(&app_handle)?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error building application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Regular);

    app.run(|_app_handle, _event| match &_event {
        RunEvent::ExitRequested { api, code, .. } => {
            // Keep the event loop running even if all windows are closed
            // This allow us to catch tray icon events when there is no window
            // if we manually requested an exit (code is Some(_)) we will let it go through
            match code {
                Some(code) => println!("got code {code}"),
                None => println!("got no code"),
            };
            if code.is_none() {
                api.prevent_exit();
            }
        }
        RunEvent::WindowEvent {
            event: tauri::WindowEvent::CloseRequested { api, .. },
            label,
            ..
        } => {
            println!("closing window... {label}");
            // run the window destroy manually just for fun :)
            // usually you'd show a dialog here to ask for confirmation or whatever
            api.prevent_close();
            _app_handle
                .get_webview_window(label)
                .unwrap()
                .hide()
                .unwrap();
        }
        _ => {}
    })
}

//https://github.com/tauri-apps/tauri/blob/dev/examples/api/src-tauri/src/tray.rs
