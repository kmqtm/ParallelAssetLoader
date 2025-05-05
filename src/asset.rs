use std::time::Duration;
use std::thread;

/// 疑似アセット構造体
pub struct Asset {
    pub id: usize,
}

impl Asset {
    /// 疑似的な読み込み処理（50msスリープ）
    pub fn load(&self) {
        println!("Loading asset {}", self.id);
        thread::sleep(Duration::from_millis(50));
    }
}
