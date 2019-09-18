use std::thread::sleep;
use std::time::Duration;

use cpu_time::{ProcessTime, ThreadTime};

#[test]
fn process_time() {
    let time = ProcessTime::now();
    sleep(Duration::new(1, 0));
    let elapsed = time.elapsed();
    assert!(elapsed < Duration::from_millis(100));
}

#[test]
fn thread_time() {
    let time = ThreadTime::now();
    sleep(Duration::new(1, 0));
    let elapsed = time.elapsed();
    assert!(elapsed < Duration::from_millis(100));
}
