use std::{thread, time};

pub fn sleep(seconds: u64) {
    let end_time = time::Instant::now() + time::Duration::from_secs(seconds);

    while time::Instant::now() < end_time {
        display_time_left(end_time);
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn display_time_left(end_time: time::Instant) {
    let time_left = end_time - time::Instant::now();
    println!("{}h, {}m, {}s", time_left.as_secs()/3600, time_left.as_secs()/60%60, time_left.as_secs()%60);
}