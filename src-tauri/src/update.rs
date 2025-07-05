use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately
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
        let answer = app
            .dialog()
            .message("An update is available. Would you like to install it??")
            .title("Update Available")
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Install".to_string(),
                "Cancel".to_string(),
            ))
            .blocking_show();
        if answer {
            update.install(downloaded_update)?;
            println!("update installed");
        }
        app.restart();
    }

    Ok(())
}
