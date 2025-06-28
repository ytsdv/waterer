use std::{error::Error, sync::Mutex as SyncMutex};

use tauri::{
    menu::{AboutMetadataBuilder, IsMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconEvent, TrayIconId},
    AppHandle, EventLoopMessage, Manager, UserAttentionType, Wry,
};
use tokio::{sync::Mutex, time::Duration};
mod db;
use db::init_db;
mod notification;
mod settings;
mod sip;

use crate::{
    db::DatabaseState,
    notification::notify_sip,
    settings::AppSettings,
    sip::{get_sips, Sip, SipState},
};

struct AppState {
    timer_started: bool,
    tray_menu_open: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            timer_started: false,
            tray_menu_open: false,
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

    fn open_tray_menu(&mut self) {
        self.tray_menu_open = true
    }

    fn close_tray_menu(&mut self) {
        self.tray_menu_open = false
    }
}

fn toggle_window_visibility(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap() {
            window.hide().unwrap();
        } else {
            window.show().unwrap();
            window
                .request_user_attention(Some(UserAttentionType::Informational))
                .unwrap_or_default();
        }
    }
}

fn get_menu_with_items(
    app_handle: &AppHandle,
    total_sip_amount_today: i64,
    timer_started: bool,
) -> Result<Menu<Wry>, tauri::Error> {
    let menu_item_show = MenuItem::with_id(app_handle, "show", "Show", true, None::<&str>)?;
    let menu_separator = PredefinedMenuItem::separator(app_handle)?;

    // Dynamic menu item based on timer state
    let timer_text = if timer_started {
        "Stop Timer"
    } else {
        "Start Timer"
    };
    let menu_item_timer = MenuItem::with_id(app_handle, "start", timer_text, true, None::<&str>)?;

    // Dynamic sip menu item with current state info
    let sip_text = format!("Sip ({}ml today)", total_sip_amount_today); // You'll need to add this getter
    let menu_item_sip = MenuItem::with_id(app_handle, "sip", &sip_text, true, None::<&str>)?;

    let menu_item_quit = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;

    // Create new menu
    Menu::with_items(
        app_handle,
        &[
            &menu_item_show,
            &menu_separator,
            &menu_item_timer,
            &menu_item_sip,
            &menu_separator,
            &menu_item_quit,
        ],
    )
}

fn update_tray_menu(
    app_handle: &AppHandle,
    timer_started: bool,
    sip_state: &SipState,
) -> Result<(), Box<dyn Error>> {
    let tray = app_handle.tray_by_id("main");

    if tray.is_none() {
        return Err("test".into());
    }
    let tray = tray.unwrap();

    // Create new menu
    let new_menu = get_menu_with_items(app_handle, sip_state.total_amount_today, timer_started)?;

    // Update the tray menu
    tray.set_menu(Some(new_menu))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_sips])
        .setup(|app| {
            let app_settings = AppSettings::load();
            app.manage(app_settings);

            // Minimize the main window on startup
            if let Some(window) = app.get_webview_window("main") {
                //let _ = window.minimize();
                //let _ = window.hide();
            }

            println!("tray icon built");

            init_db();

            let (db_state, sip_state) = tauri::async_runtime::block_on(async move {
                let database = db::Database::new()
                    .await
                    .expect("failed to initialize database");

                // Clone the pool before moving it
                let cloned_pool = database.pool.clone();

                let sip_state = SipState::new().read_from_db(&cloned_pool).await;

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
                    update_tray_menu(&app_handle, timer_started, &locked_sip_state);

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

            let menu = get_menu_with_items(&app_handle, 0, false)?;

            _ = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip(app.package_info().name.clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    "show" => {
                        println!("show menu item was clicked");
                        //println!("{:#?}", app.get_webview_window("main").unwrap());
                        toggle_window_visibility(app);
                    }
                    "start" => {
                        println!("start menu item was clicked");
                        let app_state = app.state::<SyncMutex<AppState>>();
                        let mut app_state = app_state.lock().unwrap();
                        if app_state.timer_started {
                            app_state.stop_timer();
                        } else {
                            app_state.start_timer();
                        }
                    }
                    "sip" => {
                        println!("sip menu item was clicked");
                        let db_state = app.state::<DatabaseState>();
                        let pool = &db_state.0;

                        let sip_state = app.state::<Mutex<SipState>>();

                        let result = tauri::async_runtime::block_on(async {
                            let mut locked_sip_state = sip_state.lock().await;

                            match locked_sip_state.take_sip(50, pool).await {
                                Ok(new_state) => {
                                    *locked_sip_state = new_state;
                                    println!("Updated sip state");
                                }
                                Err(e) => {
                                    eprintln!("Failed to take sip: {}", e);
                                }
                            }
                        });

                        println!("result {:#?}", result)
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .on_tray_icon_event(|icon, event| match event {
                    TrayIconEvent::DoubleClick {
                        id: _,
                        button: _,
                        position: _,
                        rect: _,
                    } => {
                        toggle_window_visibility(icon.app_handle());
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
