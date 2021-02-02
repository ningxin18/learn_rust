//1.在任意给定时间，有了可变引用之后，不能再有不可变引用
//2.引用必须有效
fn main() {
    let ref_s = dangle();
    println!("Hello, world!");

}

//无效引用，报错
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}