//1.字符串内存分配在堆上，字符串slice是String中一部分值的引用
//2.字面值就是slice
//3.其它类型slice
//ptr len cap 指针指向
fn main() {
    let s = String::from("hello world");
    let h = &s[0..5];
    let h = &s[0..=4];
    let h = &s[..5];
    println!("h = {}", h);

    let w = &s[6..];
    let w = &s[6..=10];
    let w = &s[6..11];
    println!("w = {}", w);

    let hw = &s[..];
    println!("hw = {}", hw);

//    报错
//    let ss = String::from("你好");
//    let w1 = &ss[0..1];
//    println!("s = {}", w1);

    let s2 = "hh"; //字面值,不可变引用，代码段，存在二进制 &str

    let a = [1, 2, 3, 4];
    let sss = &a[1..3];
    println!("sss = {}", sss[0]);
    println!("sss = {}", sss[1]);
    println!("sss = {}", sss.len());


}