/*
相关概念
（1）进程是资源分配的最小单位，线程是 CPU 调度的最小单位。
（2）在使用多线程时，经常会遇到的一些问题：

竞争状态：多个线程以不一致的顺序访问数据或资源；
死锁：两个线程相互等待对方停止使用其所拥有的资源，造成两者都永久等待；
只会发生在特定情况下且难以稳定重现和修复的 bug
（3）编程语言提供的线程叫做绿色线程，如 go 语言，在底层实现了 M:N 的模型，即 M 个绿色线程对应 N 个 OS 线程。但是，Rust 标准库只提供 1：1 的线程模型的实现，即一个 Rust 线程对应一个 Os 线程。

运行时代表二进制文件中包含的由语言本身提供的代码，这些代码根据语言的不同可大可小，不过非汇编语言都会有一定数量的运行时代码。通常，大家说一个语言 “没有运行时”，是指这个语言的 “运行时” 很小。Rust、C 都是几乎没有运行时的。
*/
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap(); //等待子线程结束

    for i in 1..5 {
        println!("main number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
