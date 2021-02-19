/*
弱引用Weak<T>
特点：
1.弱引用通过Rc:downgrade传递Rc实例的引用，调用Rc::downgrade会得到Weak<T>类型的智能指针，同时将weak_count加1（不是将strong_count加1）。
2.区别在于weak count无需计数为0就能使Rc实例被清理，只要strong_count为0就可以
3.可以通过Rc::upgrade方法返回Option<Rc<T>>对象。
*/

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil=> None,
        }
    }
}

//用Weak创建树形数据结构
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::List::{Cons,Nil};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!("1, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("1, a tail = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Weak::new())));
    if let Some(link) = b.tail() {
        *link.borrow_mut() = Rc::downgrade(&a);
    }
    println!("2, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("2, b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("2, b tail = {:?}", b.tail());

    //a指向b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }
    println!("3, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("3, b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("3, a tail = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //叶子节点指向父节点
    println!("Count branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    println!("Count leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
