//1.rust错误类别：可恢复错误和不可恢复错误
//1.1 可恢复错误通常代表向用户报告错误和重试操作是合理的情况，例如未找到文件。rust中使用Result<T,E>来实现
//1.2 不可恢复错误是bug的同义词，如数组的越界访问，rust中通过panic!来实现
//2.panic
//3.使用BACKTRACE=1, 打印堆栈， 使用： RUST_BACKTRACE=1 cargo run
//4.Result<T,E>
// enum Result<T,E> {
//  Ok(T),
//  Err(E),
// }
//5.简写

//当编写一个函数，但是该函数可能会失败，此时除了在函数中处理错误外，还可以将错误传给调用者，
//让调用者决定如何处理，这被称为传播错误
//什么时候用panic!, 什么时候用Result
// 1) 示例，代码原型，测试用panic!\unwrap\expece
// 2) 实际项目用Result

//use std::fs::File;
use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    //    panic!("panic here!")

//    let f = File::open("hello.txt");
//    let r = match f {
//        Ok(file) => file,
//        Err(error) => panic!("error: {:?}", error)
//    };

//    let f = File::open("hello.txt").unwrap();

//    let f = File::open("hello.txt").expect("Failed to open hello.txt");

//    let r = read_username_from_file();
//    match r {
//        Ok(s) => println!("s = {}", s),
//        Err(e) => println!("err = {:?}", e),
//    }

    let r = read_username_from_file3();
    match r {
        Ok(s) => println!("s = {}", s),
        Err(e) => println!("err = {:?}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

