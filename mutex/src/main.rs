/*
1、任何编程语言中的通道都类似于单所有权的方式，即一旦一个值传送到通道中，（发送者）将无法再使用这个值；
而共享内存就类似于多所有权，即多个线程可以同时访问相同的内存位置。

2、互斥器
（1）作用：任意时刻，只允许一个线程访问某些数据；
（2）互斥器的使用：使用前需要获取锁；使用后需要解锁数据。

3、Rust中互斥器 API：Mutex
*/

use std::sync::Mutex;
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } //离开作用域，Mutex<T>的锁会自动释放
    println!("m = {:?}", m);
}

/*
说明：
（1）Mutex 是一个智能指针，更准确的说，lock调用返回一个叫做MutexGuard的智能指针；
（2）内部提供了Drop方法，实现当MutexGuard离开作用域时自动释放锁。
*/