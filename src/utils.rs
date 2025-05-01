use std::time::Instant;

// 処理時間の測定
pub fn measure_time<F>(func: F) -> u128
where
    F: FnOnce(),
{
    let start = Instant::now();
    func();
    start.elapsed().as_millis()
}