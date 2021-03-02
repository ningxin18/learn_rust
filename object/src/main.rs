/*
1、面向对象的几个特性：对象、封装、继承。

2、对象
（1）面向对象的程序由对象组成。一个对象包含数据和操作这些数据的过程。
（2）在 Rust 中，结构体和枚举类型通过impl块提供方法。
*/

struct Dog {
    name: String,
}

impl Dog {
    fn print_name(&self) {
        println!("dog name: {}", self.name);
    }
}

fn main() {
    let d = Dog{name: String::from("wangcai")};
    d.print_name();
}
