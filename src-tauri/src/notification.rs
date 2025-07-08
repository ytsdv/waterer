use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

pub fn notify_sip(app: &AppHandle) -> anyhow::Result<()> {
    app.notification()
        .builder()
        .title("Sip is due")
        .body("Take a sip")
        .show()?;

    Ok(())
}
