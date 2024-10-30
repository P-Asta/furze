use std::fs::{self, File};
use std::io::BufReader;
use std::sync::Mutex;
static PROCESS: Mutex<f32> = Mutex::new(0.0);

#[tauri::command]
pub fn extract_zip(path: String) {
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
        {
            let mut p = PROCESS.lock().unwrap();
            *p = i as f32 / archive.len() as f32 * 100.0;
        }
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
    {
        let mut p = PROCESS.lock().unwrap();
        *p = 100.0;
    }
}

#[tauri::command]
pub fn get_process() -> f32 {
    let p = PROCESS.lock().unwrap();
    *p
}
