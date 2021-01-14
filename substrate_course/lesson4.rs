use std::{f64::consts::PI, convert::Into};
use std::ops::{Mul, Add};
use std::fmt::Display;

///////////1.信号灯////////////////
//信号灯枚举
enum TrafficLight{
    Red,
    Green,
    Yellow,
}

//信号灯，trait泛型，定义一个返回时间的方法
impl TrafficLight {
    fn time(&self) -> String {
        match &self {
            TrafficLight::Red => String::from("2021-01-11"),
            TrafficLight::Green => String::from("2021-01-11"),
            TrafficLight::Yellow => String::from("2021-01-11"),
        }
    }
}

///////////2.u32整数集合求和////////////////
fn sum(list: &[u32]) -> Option<u32> {
    if list.len() as u32 > u32::MAX {
        return None;
    }
    let mut s:u32 = 0;
    for item in list {
        s += item;
    }
    Some(s)
}

///////////3.打印图形面积////////////////
pub trait Summary {
    fn my_area(&self) -> f64;
}

struct Round<T> {
    r: T,
}

impl<T: Into<f64> + Copy + Mul<Output = T> + Display> Summary for Round<T> {
    fn my_area(&self) -> f64 {
        let r_square = self.r * self.r;
        let area = T::into(r_square) * PI;
        println!("圆形面积：{}", area);
        area
    }
}

struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Into<f64> + Add<Output = T> + Copy> Summary for Triangle<T> {
    fn my_area(&self) -> f64 {
        let length = T::into(self.a) + T::into(self.b) + T::into(self.c);
        let p = length / 2.0;
        let area = (p * (p - T::into(self.a)) * (p - T::into(self.b)) * (p - T::into(self.c))).sqrt();
        println!("三角形面积：{}", area);
        area
    }
}

struct Rectangle<T> {
    width: T,
    height: T,
}

impl <T: Into<f64> + Add<Output = T> + Copy> Summary for Rectangle<T> {
    fn my_area(&self) -> f64 {
        let area = T::into(self.width) * T::into(self.height);
        println!("正面积：{}", area);
        area
    }
}

// 打印形状类型的函数
fn compute_area<T: Summary>(shape: T) {
    shape.my_area();
}

fn main() {
    ///////////1.信号灯////////////////
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("red time is {}", red.time());
    println!("green time is {}", green.time());
    println!("yellow time is {}", yellow.time());

    ///////////2.u32整数集合求和////////////////
    let list = vec![1,2,3,4,5,6,7,8,10];
    println!("SUM: {:?}", sum(&list));

    ///////////3.打印图形面积////////////////
    // 矩形
    let rect = Rectangle { width: 1.1, height: 1.2};
    compute_area(rect);

    // 三角形
    let tri = Triangle {a: 1.0, b: 2.0, c: 3.0};
    compute_area(tri);

    // 圆形
    let round1 = Round {r: 1.0};
    compute_area(round1);
}