mod asset_loader;
mod utils;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // アセットのファイルパス（模擬データ）
    let files = vec![
        "assets/asset_01.txt",
        "assets/asset_02.txt",
        "assets/asset_03.txt",
    ];

    // チャネルを作成
    let (tx, rx) = mpsc::channel();

    // アセットローダ用スレッドを生成
    let files_shared = Arc::new(Mutex::new(files));
    let tx_clone = tx.clone();
    thread::spawn(move || {
        asset_loader::load_assets(files_shared, tx_clone);
    });

    // 送信側のクローズ(tx, rxのチャネルを閉じることになる)
    drop(tx);

    // メインスレッドで結果を受け取り
    for received in rx {
        println!("Asset Loaded: {}", received);
    }

    println!("All assets loaded, program exiting.");
}