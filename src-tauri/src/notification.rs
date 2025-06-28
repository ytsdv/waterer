use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

pub fn notify_sip(app: &AppHandle) -> Result<(), tauri_plugin_notification::Error> {
    app.notification()
        .builder()
        .title("Sip is due")
        .body("Take a sip")
        .show()
}
