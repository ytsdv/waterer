use std::sync::Mutex as SyncMutex;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    AppHandle, Manager,
};
use tokio::sync::Mutex;

use crate::{db::DatabaseState, sip::SipState, AppState};

// Global storage for menu items so they can be updated from anywhere
static MENU_ITEMS: SyncMutex<Option<(MenuItem<tauri::Wry>, MenuItem<tauri::Wry>)>> =
    SyncMutex::new(None);

pub fn update_timer_menu_item(
    _app_handle: &AppHandle,
    timer_started: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(guard) = MENU_ITEMS.lock() {
        if let Some((timer_item, _)) = guard.as_ref() {
            let text = if timer_started {
                "Stop Timer"
            } else {
                "Start Timer"
            };
            timer_item.set_text(text)?;
        }
    }
    Ok(())
}

pub fn update_sip_menu_item(
    _app_handle: &AppHandle,
    total_amount: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(guard) = MENU_ITEMS.lock() {
        if let Some((_, sip_item)) = guard.as_ref() {
            let text = format!("Sip ({}ml today)", total_amount);
            sip_item.set_text(&text)?;
        }
    }
    Ok(())
}

pub fn create_tray(app_handle: &AppHandle) -> tauri::Result<()> {
    let timer_started = {
        let app_state = app_handle.state::<SyncMutex<AppState>>();
        let app_state = app_state.lock().unwrap();
        app_state.timer_started
    };

    let menu_item_show = MenuItem::with_id(app_handle, "show", "Show", true, None::<&str>)?;
    let menu_separator = PredefinedMenuItem::separator(app_handle)?;

    // Dynamic menu item based on timer state
    let timer_text = if timer_started {
        "Stop Timer"
    } else {
        "Start Timer"
    };
    let menu_item_timer = MenuItem::with_id(app_handle, "start", timer_text, true, None::<&str>)?;

    let total_sip_amount_today = {
        let sip_state = app_handle.state::<Mutex<SipState>>();
        let locked_sip_state = tauri::async_runtime::block_on(async { sip_state.lock().await });
        locked_sip_state.total_amount_today
    };

    // Dynamic sip menu item with current state info
    let sip_text = format!("Sip ({}ml today)", total_sip_amount_today);
    let menu_item_sip = MenuItem::with_id(app_handle, "sip", &sip_text, true, None::<&str>)?;

    let menu_item_quit = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;

    // Store menu items globally for later updates
    if let Ok(mut guard) = MENU_ITEMS.lock() {
        *guard = Some((menu_item_timer.clone(), menu_item_sip.clone()));
    }

    // Create new menu
    let menu = Menu::with_items(
        app_handle,
        &[
            &menu_item_show,
            &menu_separator,
            &menu_item_timer,
            &menu_item_sip,
            &menu_separator,
            &menu_item_quit,
        ],
    )?;

    let tray_on_left_click = tauri_plugin_os::platform() == "macos";

    let app_handle = app_handle.clone();

    _ = TrayIconBuilder::with_id("main")
        .icon(app_handle.default_window_icon().unwrap().clone())
        .tooltip(app_handle.package_info().name.clone())
        .menu(&menu)
        .show_menu_on_left_click(tray_on_left_click)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            "show" => {
                println!("show menu item was clicked");
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "start" => {
                use tauri::Emitter;

                println!("start menu item was clicked");
                let app_state = app.state::<SyncMutex<AppState>>();
                let mut app_state = app_state.lock().unwrap();
                if app_state.timer_started {
                    app_state.stop_timer();
                } else {
                    app_state.start_timer();
                }

                // Update the tray menu item
                if let Err(e) = update_timer_menu_item(&app, app_state.timer_started) {
                    eprintln!("Failed to update timer menu item: {}", e);
                }

                if let Err(e) = app.emit("update-app-state", app_state.clone()) {
                    eprintln!("Failed to update app_state: {}", e);
                }
            }
            "sip" => {
                println!("sip menu item was clicked");
                let db_state = app.state::<DatabaseState>();
                let pool = &db_state.0;

                let result = tauri::async_runtime::block_on(async {
                    let sip_state = app.state::<Mutex<SipState>>();
                    let mut locked_sip_state = sip_state.lock().await;

                    match locked_sip_state.take_sip(50, pool).await {
                        Ok(new_state) => {
                            *locked_sip_state = new_state;
                            println!("Updated sip state");

                            if let Err(e) =
                                update_sip_menu_item(&app, locked_sip_state.total_amount_today)
                            {
                                eprintln!("Failed to update tray menu: {}", e);
                            }
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
            TrayIconEvent::Click {
                button_state: MouseButtonState::Up,
                button: MouseButton::Left,
                ..
            } => {
                let app_handle = icon.app_handle();
                match app_handle.get_webview_window("main") {
                    Some(window) => {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    None => {
                        eprintln!("No window found");
                        let _webview = tauri::WebviewWindowBuilder::new(
                            app_handle,
                            "main",
                            tauri::WebviewUrl::App("index.html".into()),
                        )
                        .title("Tauri")
                        .build()
                        .unwrap();
                    }
                }
            }
            _ => {}
        })
        .build(&app_handle)?;

    Ok(())
}
