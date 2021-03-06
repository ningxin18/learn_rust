//1、内部可变性：允许在使用不可变引用时改变数据。

//2、通过 RefCell 在运行时检查借用规则（通常情况下，是在编译时检查借用规则），RefCell 代表其数据的唯一所有权。
//类似于 Rc，RefCell 只能用于单线程场景。

//3、选择 Box、Rc 或 RefCell 的理由：
//Rc 允许相同数据有多个所有者；Box 和 RefCell 有单一所有者。
//Box 允许在编译时执行不可变或可变借用检查；Rc 仅允许在编译时执行不可变借用检查；RefCell 允许在运行时执行不可变或可变借用检查。
//因为 RefCell 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell 自身是不可变的情况下修改其内部的值。

//4、内部可变性：不可变值的可变借用

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a)); //不可变引用
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a)); //不可变引用

    *value.borrow_mut() += 10;   //可变，运行时成可变引用

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
