#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    use tauri_plugin_global_shortcut::{
                        Code, ShortcutState,
                    };

                    if shortcut.key == Code::F11
                        && event.state() == ShortcutState::Pressed
                    {
                        if let Some(window) = app.get_webview_window("main") {
                            let fullscreen =
                                window.is_fullscreen().unwrap_or(false);

                            let _ = window.set_fullscreen(!fullscreen);
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            use tauri_plugin_global_shortcut::{
                Code, GlobalShortcutExt, Shortcut,
            };

            app.global_shortcut()
                .register(Shortcut::new(None, Code::F11))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
