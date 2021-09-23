use std::rc::Rc;
use std::cell::RefCell;

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

#[derive(Debug)]
enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}
use ListRc::{ConsRc, NilRc};

pub fn rc_example() {
    // Rc<T> : smartpointer for reference counter
    //         data is located in heap, and Rc<T> count it's onwer
    // WARNING : Rc<T> will be used only in 'multi-thread' context
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    // this wil be compile error, because 'a' is moved to 'b' and 'b' owns 'a'
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // next, you will consider to obtain it's reference
    let b_a = &a;
    let c_a = &a;
    println!("b_a : {:?}", b_a);
    println!("c_a : {:?}", c_a);

    // This code cannot compile due to mismatch type Cons(i32, Box<&List>) /= Cons(i32, Box<List>)
    // let b = Cons(3, Box::new(b_a));
    // let b = Cons(4, Box::new(c_a));

    // you redefine ListRc, and use Rc::clone(&ref) to increment Rc reference counter and clone 'a'
    // (but actualy don't copy 'a')
    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("a (ref count) : {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a));
    let c = ConsRc(4, Rc::clone(&a));
    println!("b (Rc) : {:?}", b);
    println!("c (Rc) : {:?}", c);
    println!("a (ref count) : {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum ListRcRefCell {
    ConsRcRefCell(Rc<RefCell<i32>>, Rc<ListRcRefCell>),
    NilRcRefCell,
}
use ListRcRefCell::{ConsRcRefCell, NilRcRefCell};

// RefCall : internal mutability, apply borrowing rule in 'runtime', not in 'compile time'
pub fn refcall_example1() {
    // this code will be compile error because of definition of variable 'x' is immutable, but y
    // defined as mutable reference
    // let x = 5;
    // let y = &mut x;
    let x = 5;
    let y = RefCell::new(x);
    println!("before mut calc... : x, y = {}", x);
    *y.borrow_mut() += 10;
    // this will be output as x = 5, y = 15 !!!
    println!("after mutating... : x, y = {}, {}", x, y.borrow());

    // Rc + RefCell
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(ConsRcRefCell(Rc::clone(&value), Rc::new(NilRcRefCell)));
    let b = ConsRcRefCell(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = ConsRcRefCell(Rc::new(RefCell::new(10)), Rc::clone(&a));
    println!("before borrow_mut: a = {:?}", a);
    println!("before borrow_mut: b = {:?}", b);
    println!("before borrow_mut: c = {:?}", c);
    *value.borrow_mut() += 10;
    println!("after borrow_mut: a = {:?}", a);
    println!("after borrow_mut: b = {:?}", b);
    println!("after borrow_mut: c = {:?}", c);
}
