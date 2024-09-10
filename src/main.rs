use sysinfo::{ System, SystemExt, CpuExt };
use std::time::Duration;
use std::thread;

fn main() {
    // システム情報を初期化
    let mut sys = System::new_all();

    // モニタリング回数を設定
    let mut count = 0;
    let max_count = 20; // 20回モニタリング

    // 20回までループ
    while count < max_count {
        // システム情報をリフレッシュ
        sys.refresh_all();

        // CPU情報の取得
        let cpus = sys.cpus();
        for (i, cpu) in cpus.iter().enumerate() {
            println!("CPU {}: 使用率: {:.2}%", i + 1, cpu.cpu_usage());
        }

        // メモリ情報の取得
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        println!("総メモリ: {} MB", total_memory / (1024 * 1024));
        println!("使用中メモリ: {} MB", used_memory / (1024 * 1024));

        // 1秒待機
        println!("----------------------");
        thread::sleep(Duration::from_secs(1));

        // カウンタを増加
        count += 1;
    }

    println!("モニタリング終了");
}
