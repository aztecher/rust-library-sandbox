

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
