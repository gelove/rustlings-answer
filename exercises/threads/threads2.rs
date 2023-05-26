// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let start_at = Instant::now();
    let status = Arc::new(RwLock::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            status_shared.write().unwrap().jobs_completed += 1;
            println!("elapsed => {}", start.elapsed().as_micros());
        });
        handles.push(handle);
    }
    println!("consume => {}", start_at.elapsed().as_micros());

    // 可以注释for循环代码块
    for handle in handles {
        println!("jobs completed {}", status.read().unwrap().jobs_completed);
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        println!("jobs completed {}", status.read().unwrap().jobs_completed);
    }
    println!("consume => {}", start_at.elapsed().as_micros());
    thread::sleep(Duration::from_millis(1000));
}
