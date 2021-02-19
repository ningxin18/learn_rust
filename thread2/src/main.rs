use std::thread;
use std::time::Duration;

//fn main() {
//    let v = vec![1, 2, 3];
//    let handle = thread::spawn(|| {
//        thread::sleep(Duration::from_millis(1));
//        println!("v: {:?}", v);
//    });
//    drop(v);
//    handle.join().unwrap();
//}

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));
        println!("v: {:?}", v);
    });

    //    主线程不能再使用v
    handle.join().unwrap();
}