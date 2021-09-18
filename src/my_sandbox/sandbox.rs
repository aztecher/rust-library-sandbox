

pub fn ownership() {
    // create data in heap
    let mut s = String::from("Hello, World");
    // you can use 's' in here
    println!("before modify: s = {}", s);

    // create mutable reference of s
    // mutable borrows occcures in here, so s cannot used after this code
    let s1 = &mut s;
    // println!("before modify: s = {}", s); // this code occures compile error
    println!("before modify: s1 = {}", s1);

    // s.push_str("modify s"); // this code cannot compile
    s1.push_str("modify by s1"); // you can modify s1 because of it's mutability
    println!("after modify: s1 = {}", s1); // and you can use s1

    // s and s1 cannot modify at the same time,
    // when creating s1 (mutable reference), mutable borrow occures and s is invalid after this.
    // so, modification will occures from s1, safety
}

pub fn slice() {
    // create data in heap
    let s = String::from("Hello, World");
    // let word = _first_word(s)) // this code compile error because of mismatch argument type String vs &str
    let word = _first_word(&s[..]); // s[..] :: &str and use _first_word function to String
    println!("String converted &str result : {}", word);

    let lit = "hello, world"; // this variable type is &str ('immutable' reference)
    let word = _first_word(lit);
    println!("literal (default type is &str) result : {}", word);
}

fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

struct MyStruct {
    first: u32,
}

impl MyStruct {
    fn test_move(self) -> u32 {
        self.first
    }
    fn test_reference_val(&self) -> u32 {
        self.first
    }
    fn test_reference_refval(&self) -> &u32 {
        &self.first
    }
}

pub fn test_mystruct_reference() {
    let s = MyStruct { first: 1 };

    // // Pattern: 1
    // // moved after test_reference_move because ownership was moved when self.first in that
    // // function
    // let v = s.test_move();
    // let val = s.test_reference_val();
    // // bellow code was compile error
    //
    //
    // // Pattern : 2
    // // MyStruct.first is not after test_reference_move because *self is only reference
    // // but, test_reference_value requires Copy Traits to copy it's data
    let val = s.test_reference_val();
    let refval = s.test_reference_refval();
    assert_eq!(1, val);
    assert_eq!(1, s.first);
    assert_eq!(1, *refval);
    println!("val = {}, s.first = {}, refval = {}", val, s.first, refval);
}

struct MyAloneTuple<T>(T);

impl <T> MyAloneTuple<T> where T: Copy {
    fn new(x: T) -> MyAloneTuple<T> {
        MyAloneTuple(x)
    }
    fn test_reference_val(&self) -> T {
        self.0
    }
    fn test_reference_refval(&self) -> &T {
        &self.0
    }
}


pub fn test_myalonetuple_reference() {
    let t = MyAloneTuple::new(1);

    let val = t.test_reference_val();
    let refval = t.test_reference_refval();

    assert_eq!(1, val);
    assert_eq!(1, *refval);
    println!("val = {}, refval = {}", val, refval);
}
