use std::sync::{Arc, Mutex};
// Arc   : Type of thread safe refernce counter
// Mutex : Type of Mutex
use std::thread;


// thread function
fn some_func(lock: Arc<Mutex<u64>>) {
    loop {
        // Get lock / this code will require to get internal value
        // TODO
        // if you have panic when is getting lock, then lock() returns PoisonError and this code ignore this case by
        // using unwrap() function (panic!)
        // so, if you have to handle error pattern to implement retry
        let mut val = lock.lock().unwrap();
        // exit this thread loop when count >= 10000
        if *val >= 10000 {
            break;
        }
        // increment count
        *val += 1;
        println!("{}", *val);

    } // lock is automatically free when exiting its scope.
}

pub fn mutex_sample() {
    // Arc is thread safe reference counter (smartpointer)
    // create variable for mutex, initial value is '0'
    let lock0 = Arc::new(Mutex::new(0));

    // clone Arc value to increment reference counter (internal data is not cloned)
    let lock1 = lock0.clone();

    // create thread / lock0 was moved into closure 
    // (if not so, this will compile error because when lock0's liveness are not confirmed)
    let th0 = thread::spawn(move || {
        some_func(lock0);
    });

    // create thread
    let th1 = thread::spawn(move || {
        some_func(lock1);
    });

    // wait
    th0.join().unwrap();
    th1.join().unwrap();
}
