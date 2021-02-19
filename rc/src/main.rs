//通过Rc<T>允许程序的多个部分之间只读的共享数据，因为相同位置的多个可变饮用可能会
//造成数据竞争的不一致
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));   //此处使用a.clone()也是ok的
    println!("count after bind to b, a count = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after bind to c, a count = {}", Rc::strong_count(&a));
    }

    println!("count at end, a count = {}", Rc::strong_count(&a));

}
