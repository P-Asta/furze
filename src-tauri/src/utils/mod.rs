pub mod extract;

#[tauri::command]
pub fn select_path() -> String {
    #[cfg(target_os = "macos")]
    {
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

#[tauri::command]
pub fn get_os() -> String {
    #[cfg(target_os = "windows")]
    return "windows".to_string();

    #[cfg(target_os = "macos")]
    return "macos".to_string();

    #[cfg(target_os = "linux")]
    return "linux".to_string();

    "idk".to_string()
}
