mod asset;
mod loader;
mod utils;

use asset::Asset;
use loader::{load_assets_sequential, load_assets_parallel};
use utils::measure_time;

fn main() {
    let asset_count = 50;

    // アセットのリストを作成
    let assets: Vec<Asset> = (0..asset_count).map(|id| Asset { id }).collect();

    println!("--- Sequential ---");
    let time_seq = measure_time(|| load_assets_sequential(&assets));
    
    println!("\n--- Parallel ---");
    let time_par = measure_time(|| load_assets_parallel(&assets));

    /// 最終結果
    println!();
    println!("--- Results ---");
    println!("Sequential time:  {} ms", time_seq);
    println!("Parallel time:    {} ms", time_par);
}
