//错误原因：不能将 counter 锁的所有权移动到多个线程中。
//fn main() {
//    let counter = Mutex::new(0);
//    let mut handles = vec![];
//
//    for _ in 0..10 {
//        let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//            *num += 1;
//        });
//        handles.push(handle);
//    }
//
//    for handle in handles {
//        handle.join().unwrap();
//    }
//    println!("Result: {}", *counter.lock().unwrap());
//}

//错误原因：Rc不是线程安全的, `Rc<Mutex<i32>>` cannot be sent between threads safely
//use std::rc::Rc;
//use std::sync::Mutex;
//use std::thread;
//fn main() {
//    let counter = Rc::new(Mutex::new(0));
//    let mut handles = vec![];
//    for _ in 0..10 {
//        let cnt = Rc::clone(&counter);
//        let handle = thread::spawn(move || {
//            let mut num = cnt.lock().unwrap();
//            *num += 1;
//        });
//        handles.push(handle);
//    }
//    for handle in handles {
//        handle.join().unwrap();
//    }
//    println!("Result: {}", *counter.lock().unwrap());
//}

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

/*
Arc 是一个类似于 Rc 并可以安全的用于并发环境的类型
RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
（1）Mutex<T> 提供内部可变性，类似于 RefCell<T>；
（2）RefCell/Rc 是非线程安全的，而 Mutex/Arc 是线程安全的。
*/