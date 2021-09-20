use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn mybox_example() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let b = MyBox::new("Example of type-coercions");
    mybox_deref_type_coercion(&b);
}


pub fn mybox_deref_type_coercion(s: &str) {
    println!("dereference type coercion: {}", s);
}
