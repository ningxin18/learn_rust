//1.rust中每一个引用都有其生命周期，也就是引用保持有效的作用域，大部分时候生命周期是隐含并可以推断的
//正如大部分时候类型可以推断一样
//2.生命周期的主要目标避免悬垂引用
//3.rust编译器使用借用检查器来检查生命周期是否有效

//函数中的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_str<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

//错误例子
//fn get_str2<'a>(x: &'a str, y: &str) -> &'a str {
//    let r = String::from("abc");
//    r.as_str()
//}

//结构体中的生命周期
#[derive(Debug)]
struct StructA<'a> {
    name: &'a str,
}

//生命周期省略
/*
3.编译器采用三条规则判断引用何时不需要生命周期注解，当编译器检查完这三条规则后无法计算出引用的生命周期，则会停止并生成错误
4.生命周期注解省略规则适用于fn定义一级impl块定义
a.每个引用的参数都有它自己的生命周期参数。
    一个引用参数的函数，有一个生命周期：fm foo<'a>(x: &'a i32)
    两个引用参数的函数，有两个生命周期：fn foo<'a,'b>(x: &'a i32, y: &'b i32)
    以此类推
b.如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：
    fn foo(x: &i32) -> &i32 等价于 fm foo<'a>(x: &'a i32) -> &'a i32
c.如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为&self或者&mut self, 那么self的生命周期被赋予
所有输出生命周期参数.
*/

//方法中的生命周期
struct StuA<'a> {
    name: &'a str,
}
impl<'b> StuA<'b> {
    fn do_something(&self) -> i32 {
        1
    }
    fn do_something2(&self, s: &str) -> &str {
        self.name
    }
    fn do_something3<'b1>(&self, s: &'b1 str) -> &'b1 str {
        s
    }
}

//静态生命周期
/*
定义方式：'static
存活于整个程序期间，所有的字符字面值都拥有static生命周期。
let s: &'static str = "hello";
*/

use std::fmt::Display;
fn something<'c, T: Display>(x: &'c str, y: &'c str, ann: T) -> &'c str {
    println!("ann is {}", ann);
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("abcde");
    let s2 = String::from("ab");
    let r = longest(s1.as_str(), s2.as_str());
    println!("r = {}", r);

    let ss = get_str(s1.as_str(), s2.as_str());
//    let ss2 = get_str2(s1.as_str(), s2.as_str());
    println!("ss = {}", ss);

    let sa = String::from("hello");
    let saa = StructA{name: &sa};
    println!("saa = {:#?}", saa);
    ///////////////////////////////////////////////////////////////////

    let stua = String::from("hello");
    let stua1 = StuA{name: &stua};
    println!("stua1 = {}", stua1.do_something());
    let stua2 = String::from("world");
    println!("stua2 = {}", stua1.do_something2(&stua2));
    println!("stua2 = {}", stua1.do_something3(&stua2));

    ///////////////////////////////////////////////////////////////////
    let sth = String::from("i am s1");
    let sth2 = String::from("i am s2, hello");
    let ann = 120;
    let r2 = something(sth.as_str(), sth2.as_str(), ann);
    println!("r2 = {}", r2);
}
