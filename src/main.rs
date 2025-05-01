mod asset_loader;
mod game_loop;
mod utils;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // アセットのファイルパス（模擬データ）
    let files = vec![
        "assets/asset1.txt",
        "assets/asset2.txt",
        "assets/asset3.txt",
    ];

    // チャネルを作成
    let (tx, rx) = mpsc::channel();

    // アセットローダ用スレッドを生成
    let files_shared = Arc::new(Mutex::new(files));
    let tx_clone = tx.clone();
    thread::spawn(move || {
        asset_loader::load_assets(files_shared, tx_clone);
    });

    // メインスレッドで結果を受け取り
    for received in rx {
        println!("Asset Loaded: {}", received);
    }

    // 簡易ゲームループの開始
    game_loop::start_game_loop();
}