use std::sync::{Arc, Condvar, Mutex};
use std::collections::LinkedList;

// Implement semaphore
pub struct Semaphore {
    mutex: Mutex<isize>,
    cond: Condvar,
    max: isize,
}

impl Semaphore {
    pub fn new(max: isize) -> Self {
        Semaphore {
            mutex: Mutex::new(0),
            cond: Condvar::new(),
            max,
        }
    }
    pub fn wait(&self) {
        // if count is over self.max then waiting
        let mut cnt = self.mutex.lock().unwrap();
        while *cnt >= self.max {
            cnt = self.cond.wait(cnt).unwrap();
        }
        *cnt += 1;
    }
    pub fn post(&self) {
        // decrement count
        let mut cnt = self.mutex.lock().unwrap();
        *cnt -= 1;
        if *cnt <= self.max {
            self.cond.notify_one();
        }
    }
}

// We can implement Finite Sized Channel by using Semaphore
// Channel are constructed Sender / Receiver
// [INFO]
// You can also use std::sync::mpsc::sync_channel.

#[derive(Clone)]
pub struct Sender<T> {
    sem: Arc<Semaphore>, // semaphore that represents finiteness.
    buf: Arc<Mutex<LinkedList<T>>>, // Queue
    cond: Arc<Condvar>, // conditional variable for Read
}

impl<T: Send> Sender<T> { // T requires Send trait
    // send function
    pub fn send(&self, data: T) {
        self.sem.wait(); // if reacy max size of Queue, then wait.
        let mut buf = self.buf.lock().unwrap();
        buf.push_back(data); // enqueue
        self.cond.notify_one(); // notify to receiver that can read from queue
    }
}

pub struct Receiver<T> {
    sem: Arc<Semaphore>, // semaphore that represents finiteness.
    buf: Arc<Mutex<LinkedList<T>>>, // queue
    cond: Arc<Condvar>, // onditional variable for Read
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> T {
        let mut buf = self.buf.lock().unwrap();
        loop {
            // get from queue
            if let Some(data) = buf.pop_front() {
                self.sem.post();
                return data;
            }
            // wait notification when the queue is empty
            buf = self.cond.wait(buf).unwrap();
        }
    }
}

pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);
    let sem = Arc::new(Semaphore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cond = Arc::new(Condvar::new());
    let tx = Sender {
        sem: sem.clone(),
        buf: buf.clone(),
        cond: cond.clone(),
    };
    let rx = Receiver { sem, buf, cond };
    (tx, rx)
}

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 8;

pub fn channel_sample() {
    let (tx, rx) = channel(4);
    let mut v = Vec::new();

    // receive thread
    let t = std::thread::spawn(move || {
        let mut cnt = 0;
        while cnt < NUM_THREADS * NUM_LOOP {
            let n = rx.recv();
            println!("recv: n = {:?}", n);
            cnt += 1;
        }
    });
    v.push(t);

    // send thread
    for i in 0..NUM_THREADS {
        let tx0 = tx.clone();
        let t = std::thread::spawn(move || {
            for j in 0..NUM_LOOP {
                tx0.send((i, j));
            }
        });
        v.push(t);
    }
    for t in v {
        t.join().unwrap();
    }

}
