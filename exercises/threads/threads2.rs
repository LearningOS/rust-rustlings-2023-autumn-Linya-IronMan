// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::Arc;
// NOTE: 保护在线程中分享的数据，并且内部是可变的（不管外部是否可变）
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

// NOTE: 数据竞争。读写竞争
fn main() {
    // NOTE: Arc 与 Mutex 的结合使用
    // 如果单纯访问数据 —— 使用 Arc
    // 如果需要修改数据，加上 Mutex
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // NOTE: lock 获得数据的所有权，并且可以写了
            let mut status_lock = status_shared.lock().unwrap();
            status_lock.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        let status_shared = Arc::clone(&status);
        println!(
            "jobs completed {}",
            status_shared.lock().unwrap().jobs_completed
        )
    }
}
