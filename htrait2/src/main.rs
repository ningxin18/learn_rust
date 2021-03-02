/*
默认泛型类型参数和运算符重载
（1）使用泛型类型参数时，可以为泛型指定一个默认的具体类型。
（2）运算符重载是指在特定情况下自定义运算符行为的操作。Rust 并不允许创建自定义运算符或者重载运算符，不过对于 std::ops 中列出的运算符和相应的 trait，我们可以实现运算符相关 trait 来重载。
*/

use std::ops::Add;
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });

    let mi = Millimeters(1);
    let m = Meters(1);
    let r = mi + m;
    println!("r = {:?}", r);
}

//尖括号中的 RHS=Self，这个语法叫做默认类型参数（default type parameters）。
//RHS是一个泛型类型参数（“right hand side” 的缩写），它用于定义add方法中的rhs参数。
//如果实现Add trait时不指定RHS的具体类型，RHS的类型将是默认的Self类型，在默认类型上实现的Add的类型

// 默认参数类型主要用于如下两个方面：1.扩展类型而不破坏现有代码。2.在大部分用户都不需要的特定情况进行自定义。
//trait Add<RHS=Self> {
//    type Output;
//    fn add(self, rhs: RHS) -> Self::Output;
//}

