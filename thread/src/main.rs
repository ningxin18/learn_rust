/*
相关概念
（1）进程是资源分配的最小单位，线程是 CPU 调度的最小单位。
（2）在使用多线程时，经常会遇到的一些问题：

竞争状态：多个线程以不一致的顺序访问数据或资源；
死锁：两个线程相互等待对方停止使用其所拥有的资源，造成两者都永久等待；
只会发生在特定情况下且难以稳定重现和修复的 bug
（3）编程语言提供的线程叫做绿色线程，如 go 语言，在底层实现了 M:N 的模型，即 M 个绿色线程对应 N 个 OS 线程。但是，Rust 标准库只提供 1：1 的线程模型的实现，即一个 Rust 线程对应一个 Os 线程。

运行时代表二进制文件中包含的由语言本身提供的代码，这些代码根据语言的不同可大可小，不过非汇编语言都会有一定数量的运行时代码。通常，大家说一个语言 “没有运行时”，是指这个语言的 “运行时” 很小。Rust、C 都是几乎没有运行时的。



1、有两个并发概念内嵌于语言中：std::marker 中的 Sync 和 Send trait。

2、通过 Send 允许在线程间转移所有权
（1）Send 标记 trait 表明类型的所有权可以在线程间传递。几乎所有的 Rust 类型都是 Send 的，但是例外：例如 Rc<T> 是不能 Send 的。
（2）任何完全由 Send 类型组成的类型也会自动被标记为 Send。

3、Sync 允许多线程访问
（1）Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用，即，对于任意类型 T，如果 &T（T 的引用）是 Send 的话 T 就是 Sync 的，这意味着其引用就可以安全的发送到另一个线程。
（2）智能指针 Rc<T> 也不是 Sync 的，出于其不是 Send 相同的原因。RefCell<T > 和 Cell<T> 系列类型不是 Sync 的。RefCell<T> 在运行时所进行的借用检查也不是线程安全的，Mutex<T> 是 Sync 的。

4、手动实现 Send 和 Sync 是不安全的
通常并不需要手动实现 Send 和 Sync trait，因为由 Send 和 Sync 的类型组成的类型，自动就是 Send 和 Sync 的。因为他们是标记 trait，甚至都不需要实现任何方法。他们只是用来加强并发相关的不可变性的。

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
