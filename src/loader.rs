use std::time::Instant;
use std::thread;

use crate::asset::Asset;

/// 逐次読み込み
pub fn load_assets_sequential(assets: &[Asset]) {
    let start = Instant::now();

    for asset in assets {
        asset.load();
    }

    let duration = start.elapsed();
    println!("Sequential loading took: {:?}", duration);
}

/// 並列読み込み（スレッド生成）
pub fn load_assets_parallel(assets: &[Asset]) {
    let start = Instant::now();

    let handles: Vec<_> = assets
        .iter()
        .map(|asset| {
            let asset = Asset { id: asset.id };
            thread::spawn(move || {
                asset.load();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Parallel loading took: {:?}", duration);
}
