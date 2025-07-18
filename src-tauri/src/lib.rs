use std::{
    env,
    sync::{Mutex as SyncMutex, PoisonError},
};

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Pool, Sqlite};
use tauri::{Manager, RunEvent};
use tokio::{sync::Mutex, time::Duration};
mod db;
use db::init_db;
mod notification;
mod settings;
mod sip;
mod tray;
mod update;

use uuid::Uuid;
mod state;
use state::{AppTimerState, SettingsState, SipTrackingState};

use crate::{
    db::DatabaseState,
    notification::notify_sip,
    settings::{get_settings, update_settings, AppSettings},
    sip::{get_sips, SipState},
    tray::{create_tray, update_timer_menu_item},
    update::update,
};

#[derive(Serialize, Clone)]
struct AppState {
    timer_started: bool,
    session_id: Option<i64>,
    session_start: DateTime<Utc>,
}

impl AppState {
    fn new() -> Self {
        Self {
            timer_started: false,
            session_id: None,
            session_start: Utc::now(),
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

    pub async fn init_session(&mut self, pool: &Pool<Sqlite>) -> anyhow::Result<()> {
        let session_uuid = Uuid::new_v4().to_string();
        let session_start_str = self.session_start.to_rfc3339();

        let result = sqlx::query!(
            "INSERT INTO sessions (session_id, session_start) VALUES (?, ?)",
            session_uuid,
            session_start_str
        )
        .execute(pool)
        .await?;

        self.session_id = Some(result.last_insert_rowid());

        Ok(())
    }
}

#[tauri::command]
async fn toggle_timer(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppTimerState>,
    _sip_state: tauri::State<'_, SipTrackingState>,
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

#[tauri::command]
async fn get_app_state(app_state: tauri::State<'_, AppTimerState>) -> Result<AppState, String> {
    match app_state.lock() {
        Ok(app_state) => Ok(app_state.clone()),
        Err(e) => Err(format!("Failed to fetch sips: {}", e)),
    }
}

#[tauri::command]
async fn take_sip(
    db_state: tauri::State<'_, DatabaseState>,
    sip_state: tauri::State<'_, SipTrackingState>,
    settings: tauri::State<'_, SettingsState>,
    app_state: tauri::State<'_, AppTimerState>,
) -> Result<SipState, String> {
    let pool = &db_state.0;

    let sip_amount = {
        let settings = settings.lock();
        let settings = settings.ignore_poisoned();
        settings.sip_amount_ml
    };

    let session_id = {
        let state = app_state.lock().ignore_poisoned();
        state.session_id
    };

    let session_id = match session_id {
        Some(id) => id,
        None => return Err("No session ID available".to_string()),
    };

    let mut locked_sip_state = sip_state.lock().await;

    match locked_sip_state
        .take_sip(sip_amount, pool, session_id)
        .await
    {
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
            get_app_state,
            update_settings,
            get_settings
        ])
        .setup(|app| {
            app.manage(SettingsState::new(AppSettings::load()));
            app.manage(AppTimerState::new(AppState::new()));

            //update check
            let app_handle_for_update = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(&app_handle_for_update).await.unwrap_or_else(|e| {
                    eprintln!("Failed to check for updates: {}", e);
                });
            });

            init_db();

            //blocking async init operations
            let app_handle = app.handle().clone();
            let (db_state, sip_state) = tauri::async_runtime::block_on(async move {
                let database = db::Database::new()
                    .await
                    .expect("failed to initialize database");

                // Clone the pool before moving it
                let cloned_pool = database.pool.clone();

                let sip_state = SipState::default().read_from_db(&cloned_pool).await;

                let app_state = app_handle.state::<AppTimerState>();
                let mut app_state = app_state.lock().ignore_poisoned();

                // Session creation is critical - crash if it fails
                app_state
                    .init_session(&cloned_pool)
                    .await
                    .expect("Critical error: Failed to add session to database. App cannot function without session tracking.");

                (db::DatabaseState(database.pool), sip_state)
            });

            // Store database pool in app state
            app.manage(db_state);
            app.manage(SipTrackingState::new(sip_state));

            let db_state = app.state::<DatabaseState>();

            let cloned_pool = db_state.0.clone();

            // Clone the app handle so it can be moved into the spawned task
            let app_handle = app.handle().clone();

            let _ = tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));

                loop {
                    let timer_started = {
                        let app_state = app_handle.state::<AppTimerState>();
                        let app_state = app_state.lock().unwrap();
                        app_state.timer_started
                    };

                    interval.tick().await;

                    let sip_state = app_handle.state::<SipTrackingState>();
                    let mut locked_sip_state = sip_state.lock().await;

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

pub trait IgnorePoisoned<T> {
    fn ignore_poisoned(self) -> T;
}

impl<T> IgnorePoisoned<T> for Result<T, PoisonError<T>> {
    fn ignore_poisoned(self) -> T {
        self.expect("poisoned")
    }
}
