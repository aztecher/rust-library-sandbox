use std::sync::{Arc, Mutex, Condvar};
// condvar : Conditional Variable
use std::thread;

fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    // get mutex lock
    let mut started = lock.lock().unwrap();
    while !*started { // loop while Mutex's shared variable 'false'
        // wait : simultaneously 'free lock' / 'wait notify'
        //        if started is true, then this waiting code will not need.
        started = cvar.wait(started).unwrap();
    }
    // or use wait_while(started, |started| !*started).unwrap();
    println!("child {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    // get mutex lock
    let mut started = lock.lock().unwrap();
    *started = true; // update shared varaible
    cvar.notify_all(); // notification
    println!("parent");
}

pub fn condvar_sample() {
    // create mutex / condvar
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    let c0 = thread::spawn(move || { child(0, pair0) });
    let c1 = thread::spawn(move || { child(1, pair1) });
    let p  = thread::spawn(move || { parent(pair2) });

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}
