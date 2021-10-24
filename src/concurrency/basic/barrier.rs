use std::sync::{Arc, Barrier};
use std::thread;

pub fn barrier_sample() {
    // create vector for save thread handlers
    let mut v = Vec::new();

    // create 10 thread barrier
    let barrier = Arc::new(Barrier::new(10));

    // create / start 10 thread
    for _ in 0..10 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            b.wait(); // barrier sync
            println!("finished barrier");
        });
        v.push(th);
    }
    for th in v {
        th.join().unwrap();
    }
}
