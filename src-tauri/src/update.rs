use serde::Serialize;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

#[derive(Serialize, Clone, Debug)]
pub struct UpdateInfo {
    pub available: bool,
    pub version: Option<String>,
    pub body: Option<String>,
    pub date: Option<String>,
}

// Check for updates without showing dialog
#[tauri::command]
pub async fn check_for_updates(app: tauri::AppHandle) -> Result<UpdateInfo, String> {
    match app.updater() {
        Ok(updater) => {
            match updater.check().await {
                Ok(Some(update)) => {
                    Ok(UpdateInfo {
                        available: true,
                        version: Some(update.version.clone()),
                        body: update.body.clone(),
                        date: update.date.clone().map(|d| d.to_string()),
                    })
                }
                Ok(None) => {
                    Ok(UpdateInfo {
                        available: false,
                        version: None,
                        body: None,
                        date: None,
                    })
                }
                Err(e) => Err(format!("Failed to check for updates: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to initialize updater: {}", e)),
    }
}

// Install update with progress tracking
#[tauri::command]
pub async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    match app.updater() {
        Ok(updater) => {
            match updater.check().await {
                Ok(Some(update)) => {
                    use tauri::Emitter;
                    
                    let mut downloaded = 0;
                    
                    // Show update started event
                    let _ = app.emit("update-download-started", ());
                    
                    let downloaded_update = update
                        .download(
                            |chunk_length, content_length| {
                                downloaded += chunk_length;
                                let progress = if let Some(total) = content_length {
                                    (downloaded as f64 / total as f64 * 100.0) as u32
                                } else {
                                    0
                                };
                                
                                // Emit progress event
                                let _ = app.emit("update-download-progress", serde_json::json!({
                                    "downloaded": downloaded,
                                    "total": content_length,
                                    "progress": progress
                                }));
                            },
                            || {
                                let _ = app.emit("update-download-finished", ());
                            },
                        )
                        .await
                        .map_err(|e| format!("Failed to download update: {}", e))?;

                    // Install and restart
                    update.install(downloaded_update)
                        .map_err(|e| format!("Failed to install update: {}", e))?;
                    
                    app.restart();
                    Ok(())
                }
                Ok(None) => Err("No update available".to_string()),
                Err(e) => Err(format!("Failed to check for updates: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to initialize updater: {}", e)),
    }
}

// Legacy function for backwards compatibility - now shows user choice
pub async fn update(app: tauri::AppHandle) -> anyhow::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let answer = app
            .dialog()
            .message(&format!(
                "An update to version {} is available. Would you like to install it now?",
                update.version
            ))
            .title("Update Available")
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Install Now".to_string(),
                "Later".to_string(),
            ))
            .blocking_show();

        if !answer {
            // User chose "Later" - emit event to show update button in header
            use tauri::Emitter;
            let _ = app.emit("update-available", serde_json::json!({
                "version": update.version,
                "body": update.body,
                "date": update.date.map(|d| d.to_string())
            }));
            return Ok(());
        }

        // User chose "Install Now"
        let mut downloaded = 0;
        let downloaded_update = update
            .download(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        update.install(downloaded_update)?;
        app.restart();
    }

    Ok(())
}
