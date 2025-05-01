use std::fs;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// ファイル読み込み関数
pub fn load_assets(files: Arc<Mutex<Vec<&str>>>, sender: Sender<String>) {
    let files = files.lock().unwrap();
    for file in files.iter() {
        // 模擬的なファイル読み込み時間
        thread::sleep(Duration::from_millis(500));
        
        // 読み込み処理
        let content = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(_) => "Failed to load".to_string(),
        };
        sender.send(format!("{}: {}", file, content)).unwrap();
    }
}