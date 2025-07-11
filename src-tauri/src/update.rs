use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

//TODO: check if anyhow error also gets the strings
pub async fn update(app: &tauri::AppHandle) -> anyhow::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;
        let answer = app
            .dialog()
            .message("An update is available. Would you like to install it??")
            .title("Update Available")
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Install".to_string(),
                "Cancel".to_string(),
            ))
            .blocking_show();

        println!("answer: {:?}", answer);

        if !answer {
            return Ok(());
        }
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

        update.install(downloaded_update)?;
        app.restart();
    }

    Ok(())
}
