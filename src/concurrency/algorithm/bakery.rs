use std::ptr::{read_volatile, write_volatile};
use std::sync::atomic::{fence, Ordering};
use std::thread;

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;
static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};
static mut COUNT: u64 = 0;

// macro for volatile
macro_rules! read_mem {
    ($addr: expr) => { unsafe { read_volatile($addr) } };
}

macro_rules! write_mem {
    ($addr: expr, $val: expr) => {
        unsafe { write_volatile($addr, $val) }
    };
}

// Lock management
struct LockGuard {
    idx: usize,
}

impl Drop for LockGuard {
    // lock free
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.idx], None);
    }
}

struct BakeryLock {
    entering: [bool; NUM_THREADS],
    tickets: [Option<u64>; NUM_THREADS],
}

impl BakeryLock {
    // Lock function, idx is thread number
    fn lock(&mut self, idx: usize) -> LockGuard {
        // Retrieve ticket from here
        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], true);
        fence(Ordering::SeqCst);

        // get ticket max value
        let mut max = 0;
        for i in 0..NUM_THREADS {
            if let Some(t) = read_mem!(&self.tickets[i]) {
                max = max.max(t);
            }
        }
        // max + 1 = ticket number
        let ticket = max + 1;
        write_mem!(&mut self.tickets[idx], Some(ticket));

        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], false);
        fence(Ordering::SeqCst);

        // Waiting from here
        for i in 0..NUM_THREADS {
            if i == idx {
                continue;
            }
            // if thread i entering ticket, then waiting
            while read_mem!(&self.entering[i]) {};
            loop {
                // compare thread i and my ticket
                // if my priority is high and thread i is not processing, then end of wait.
                match read_mem!(&self.tickets[i]) {
                    Some(t) => {
                        if ticket < t || (ticket == t && idx < i) {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        fence(Ordering::SeqCst);
        LockGuard { idx }
    }
}

pub fn bakery_algorithm_impl() {
    // create threads
    let mut v = Vec::new();
    for i in 0..NUM_THREADS {
        let th = thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let _lock = unsafe { LOCK.lock(i) };
                unsafe {
                    let c = read_volatile(&COUNT);
                    write_volatile(&mut COUNT, c + 1);
                }
            }
        });
        v.push(th);
    }
    for th in v {
        th.join().unwrap();
    }
    println!(
        "COUNT = {} (expected = {})",
        unsafe { COUNT },
        NUM_LOOP * NUM_THREADS
    );
}
