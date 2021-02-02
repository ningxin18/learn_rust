const MAX_POINT: u32 = 100000;

fn main() {
    //1.变量定义用let, 如果变量没有mut，那么是不可变的
    let a = 1;
    println!("a = {}", a);

    let b: u32 = 1;
    println!("b = {}", b);

    // 不能对不可变变量赋值两次
    // b = 2;    Cannot assign twice to immutable variable
    let mut c: u32 = 1;
    c = 3;
    println!("c = {}", c);

    // 2.隐藏性
    let c: f32 = 1.1;
    println!("c = {}", c);

    // 3.常量
    println!("MAX_POINT = {}", MAX_POINT);
}
