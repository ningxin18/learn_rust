use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
       let val = String::from("hi");
        tx.send(val).unwrap();
//        println!("val = {}", val); //调用send时，会发生move，所以此处不能再使用
    });

    let re = rx.recv().unwrap();
    println!("got: {}", re);

}
