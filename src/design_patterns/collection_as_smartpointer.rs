use std::ops::Deref;


// more document > https://doc.rust-jp.rs/book-ja/ch15-02-deref.html

// Vec<T> represents the sets of element T.
struct Vec<T> {
    data: T,
}

impl<T> Deref for Vec<T> {
    // slice [T] owned the set of T
    type Target = [T];

    // Deref represents the dereference and check it's type coercison
    fn deref(&self) -> &[T] {
        // ..
    }
}



