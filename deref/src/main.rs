//1、实现 Deref trait 允许我们重载解引用运算符。意思就是，如果A实现了Deref trait，那么就可以写如下代码：
//let a: A = A::new(); //前提：A类型必须实现Deref trait
//let b = &a;
//let c = *b;//对A类型解引用

//自定义智能指针 MyBox：实现 Deref trait
//struct MyBox<T>(T);
//
//impl<T> MyBox<T> {
//    fn new(x: T) -> MyBox<T> {
//        MyBox(x)
//    }
//}

//实现Deref Trait
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> { //为MyBox实现Deref trait
    type Target = T;
    fn deref(&self) -> &T { //注意：此处返回值是引用，因为一般并不希望解引用获取MyBox<T>内部值的所有权
        &self.0
    }
}

fn main() {
//    通过解引用使用指针的值
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); //解引用

    let z = Box::new(x);
    assert_eq!(5, *z);

    let x1 = 5;
    let y1 = MyBox::new(x1);
    assert_eq!(5, x1);
    assert_eq!(5, *y1); //实现Deref trait后即可解引用，使用*y实际等价于 *(y.deref())

    let m = MyBox::new(String::from("rust"));
    hello(&m); //将MyBox变为&String，再将String的解引用，变为字符串slice.
    //此处解引用时，强制多态，将&String变为&str，否则的话此处需要：hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

//解引用多态与可变性交互，解引用多态有如下三种情况：
//当T: Deref<Target=U>时，从&T到&U
//当T: DerefMut<Target=U>时，从&mut T到&mut U
//当T: Deref<Target=U>时，从&mut T到&U（注意：此处反之是不可能的）
