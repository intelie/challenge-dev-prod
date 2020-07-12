use std::f64::consts::PI;
use std::u64::MAX as MAXU64;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{delay_for, Duration};

use chrono::{Local as ChronoLocal, SecondsFormat};
use rand::{thread_rng, Rng};

pub async fn process_connection(mut stream: TcpStream) {
    let address = stream.peer_addr().unwrap();
    println!("{} - Connected!", address);

    let min = thread_rng().gen_range(1, 1024);
    for i in min.. {
        let line = create_rand_line(i);

        if let Err(e) = stream.write(&line.into_bytes()).await {
            eprintln!("{} - {}", address, e);
            return;
        }

        delay_for(Duration::from_millis(500)).await;
    }
}

fn create_rand_line(i: i32) -> String {
    let now = get_time_rfc3339();
    let sin = (i as f64 * 2.0 * PI / 360.0).sin();
    let rand = create_rand_string();

    format!("{},{:04},{:.6},\"{}\"\n", now, i, sin, rand)
}

fn get_time_rfc3339() -> String {
    let now = ChronoLocal::now();
    now.to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn create_rand_string() -> String {
    let i: u64 = thread_rng().gen_range(0, MAXU64);
    format!(
        "(a={:016x?},b={:08x?}-{:08x?}::{:08x?};c={:08x?})",
        i & 0x1111111111111111,
        i >> 32,
        i & 0xFFFFFF,
        i | 0x222222,
        i >> 32 ^ 0x111111
    )
}
