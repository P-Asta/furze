// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
static mut ZIP_PATH: String = String::new();
#[tauri::command]
fn zip_path() -> String {
    unsafe { ZIP_PATH.clone() }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    unsafe {
        ZIP_PATH = format!("{:?}", std::env::args());
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![zip_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
