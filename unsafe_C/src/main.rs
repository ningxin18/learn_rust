//调用C语言的函数
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs: {}", abs(-3));
    }
}
