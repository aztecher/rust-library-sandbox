
pub fn _vector_example() {
    // Box<T> locate data to heap
    // let vec = Box::new(vec![1, 2, 3, 4, 5]);
    let vector = vec![1, 2, 3, 4, 5];
    println!("{:?}", vector);
    let result = vector.iter().map(|e| e * 2).collect::<Vec<i32>>();
    // vec.into_iter().map(|s| s * 2).collect()::;
    println!("{:?}", result);
}

// // you cannot define List as bellow because the instance of this type may be infinite size
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// but you can define as follows
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

pub fn box_example() {
    // Box::new(5) is set data '5' in heap
    let b = Box::new(5);
    // but, you can access as stack variable
    println!("b = {}", b);

    // above example is stupid, nothing the benetfit to locate these data in heap.
    // but, you can define recursive data type by using box.
    // (because it cannot detect it's size in compile time, but size of 'box' is fixed!)

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("box = {:?}", list);
}


