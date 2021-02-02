fn takes_ownership(some_string: String) {
    println!("some_string = {}", some_string);
}

fn  makes_copy(i: i32) {
    println!("i = {}", i);
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s); //函数执行完毕，s回收，不能再使用

    let x = 5;
    makes_copy(x); //函数执行完毕，可以继续使用，分配在栈上的
    println!("x = {}", x);

}
