use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use crate::List::{Cons,Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)),Rc::clone(&a));
    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c);
    println!("++++++++++++++++++++++++++++++++++");
    *value.borrow_mut() += 10;
    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c);
}
