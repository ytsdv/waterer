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
    update::update,
};

#[derive(Serialize, Clone)]
struct AppState {
    timer_started: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            timer_started: false,
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
            get_app_state
        ])
        .setup(|app| {
            let app_settings = AppSettings::load();
            app.manage(app_settings);

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(app_handle).await.unwrap_or_else(|e| {
                    eprintln!("Failed to check for updates: {}", e);
                });
            });

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

            let _ = tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));

                loop {
                    let timer_started = {
                        let app_state = app_handle.state::<SyncMutex<AppState>>();
                        let app_state = app_state.lock().unwrap();
                        app_state.timer_started
                    };

                    interval.tick().await;
                    println!("Tick");

                    let sip_state = app_handle.state::<Mutex<SipState>>();
                    let mut locked_sip_state = sip_state.lock().await;
                    //

                    if !timer_started {
                        continue;
                    }

                    if locked_sip_state.check_if_sip_is_due() {
                        println!("Sip is due");
                        println!("notified_user {:#?}", locked_sip_state.notified_user);
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
                    } else {
                        println!("Sip is not due");
                        println!("notified_user {:#?}", locked_sip_state.notified_user);
                    }
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
