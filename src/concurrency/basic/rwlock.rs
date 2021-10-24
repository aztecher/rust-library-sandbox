use std::sync::RwLock;

pub fn rwlock_sample() {
    // create RwLock instance, initial value is '10'
    let lock = RwLock::new(10);
    {
        // get immutable reference
        let v1 = lock.read().unwrap(); // for Read
        let v2 = lock.read().unwrap(); // for Read
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }
    {
        // get mutable reference
        let mut v = lock.write().unwrap();
        *v = 7;
        println!("v = {}", v);
    }
}
