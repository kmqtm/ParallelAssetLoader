use std::thread;
use std::time::{Duration, Instant};

pub fn start_game_loop() {
    const FPS: u32 = 30;
    let frame_duration = Duration::from_secs_f32(1.0 / FPS as f32);

    println!("Starting game loop at {} FPS...", FPS);

    loop {
        let frame_start = Instant::now();

        // 状態更新
        update();

        // フレームの終了時間を計算
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }
}

// 状態の更新関数
fn update() {
    println!("Game state updated.");
}