// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// static mut ZIP_PATH: String = String::new();

use cocoa::appkit::NSApplication;
use cocoa::base::{nil, selector};
use std::fs::{self, File};
use std::io::BufReader;
#[tauri::command]
fn zip_path() -> String {
    #[cfg(target_os = "macos")]
    unsafe {
        // Initialize the Cocoa application (this is required to work with macOS Finder)
        // let app = NSApplication::sharedApplication(nil);
        // app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicyRegular); // 0 for NSApplicationActivationPolicyRegular

        // Use AppleScript to get the selected file path from Finder
        let script = r#"
            tell application "Finder"
                set theFile to the selection as alias
                set thePath to POSIX path of theFile
                set the selection to {}
            end tell
            return thePath
        "#;

        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .unwrap();

        let file_path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if !file_path_str.is_empty() {
            return format!("{} {:?}", file_path_str, std::env::vars());
        }
    }
    #[cfg(target_os = "windows")]
    {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            return args[1].clone();
        }
    }
    "".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![zip_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn extract_zip(path: String) {
    // 파일 읽기
    let fname = std::path::Path::new(&path);
    let file = fs::File::open(fname).unwrap();
    let reader = BufReader::new(file);
    let mut name = path[..path.len() - 4].to_string();
    let mut i = 0;
    loop {
        match std::fs::create_dir(&name) {
            Ok(_) => break,
            Err(_) => {
                i += 1;
                name = format!("{} ({})", path[..path.len() - 4].to_string(), i);
            }
        }
    }

    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        if file.is_dir() {
            std::fs::create_dir(format!("{}/{}", name, outpath.display().to_string())).unwrap();
        } else {
            let mut outpath =
                File::create(format!("{}/{}", name, outpath.display().to_string())).unwrap();
            std::io::copy(&mut file, &mut outpath).unwrap();
        }
    }
}
