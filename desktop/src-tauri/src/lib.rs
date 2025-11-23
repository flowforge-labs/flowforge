#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![ingest_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn core_ping() -> String {
    flowforge_core::ping().to_string()
}

#[tauri::command]
fn ingest_path(path: String) -> String {
    println!("Tauri  received path: {}", path);
    // TODO: wire in core::ingest_folder(path)
    // pros and cons of handling as path/pathbuf now?
    "OK".into()
}
