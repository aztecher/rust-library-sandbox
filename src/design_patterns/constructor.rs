pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    // create new empty Vec<T> instance
    // this is static method, so no 'self' argument.
    // this constructor don't take any argument,
    // but you can pass these parameter as initialize parameter
    pub fn new() -> Vec<T> {
        Vec {
            buf: RawVec::new(),
            len: 0,
        }
    }
}
