/*
1、Rust 中一个实现消息传递并发的主要工具是通道。
通道由两部分组成，一个是发送端，一个是接收端，发送端用来发送消息，接收端用来接收消息。
发送者或者接收者任一被丢弃时就可以认为通道被关闭了。

2、通道介绍
（1）通过 mpsc::channel，创建通道，mpsc 是多个生产者，单个消费者；
（2）通过 spmc::channel，创建通道，spmc 是一个生产者，多个消费者；
（3）创建通道后返回的是发送者和消费者，示例：
*/

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();//返回发送者、接收者

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //使用发送者通过channel发送
    });

    let received = rx.recv().unwrap();//使用接收者通过channel接收
    println!("Got: {}", received);
}

/*
知识点：
（1）发送者的send方法返回一个Result<T, E> 类型，如果接收端已经被丢弃了，将没有发送值的目标，所以发送操作将返回错误；
（2）接收者的recv方法也返回Result<T, E>类型，当通道发送端关闭时，将返回一个错误值,表明不会再由新的值到来了；
（3）接收还可以使用try_recv方法，recv方法会阻塞到一直等待到消息到来，而try_recv不会阻塞，它会立即返回，Ok值标识包含可用信息，而Err则代表此时没有任何信息。
*/