// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

impl JobStatus {
    fn new(num: u32) -> JobStatus {
        JobStatus {
            jobs_completed: num,
        }
    }
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus::new(0)));
    let status_shared = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut s = status_shared.lock().unwrap();
            s.jobs_completed += 1;
        }
    });
    //let s = status.lock().unwrap();
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
