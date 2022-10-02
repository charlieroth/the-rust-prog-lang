enum List {
    Cons(i32, Rc<List>),
    Nil,
}

enum List1 {
    Cons(i32, Box<List>),
    Nil,
}

use std::rc::Rc;

fn main() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));
    
    let a_1 = List1::Cons(5, Box::new(List1::Cons(10, Box::new(List1::Nil))));
    // `a_1` is moved here
    let b_1 = List1::Cons(3, Box::new(a_1));
    // error: attempting to use borrowed value
    let c_1 = List1::Cons(4, Box::new(a_1));
}
