//    1.类似于C语言的方式定义
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//    2.rust语言提倡的定义方式
enum IpAddr2 {
    V4(String),
    V6(String),
}

//    3.可以是不同类型
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

//    4.经典用法 Move是个结构体，
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    Change(i32, i32, i32),
}
//等同于如下类型
//struct QuitMessage; //类单元结构体
//struct MoveMessage{
//  x: i32,
//  y: i32,
// }
// struct WriteMessage(String)
// struct ChangeMesage(i32, i32, i32)

//    5.枚举类型的方法以及match
impl Message {
    fn prin(&self) {
        match self {
            Message:: Quit => println!("Quit"),
            Message:: Move{x, y} => println!("Move x = {}, y = {}", x, y),
            Message:: Change(a, b, c) => println!("Change, a = {}, b = {}, c = {}", a, b, c),
//            _ => println!("Write"),
            Message:: Write(s) => println!("Writ = {}", s),
        }
    }
}

fn main() {
    let i1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let i2 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let i1 = IpAddr2::V4(String::from("127.0.0.1"));
    let i2 = IpAddr2::V6(String::from("127.0.0.1"));

    let i1 = IpAddr3::V4(127, 0, 0, 1);
    let i2 = IpAddr3::V6(String::from("127.0.0.1"));

    let quit = Message::Quit;
    quit.prin();

    let mo = Message::Move{x:10, y:20};
    mo.prin();

    let wri = Message::Write(String::from("hello"));
    wri.prin();

    let change = Message::Change(1,2,3);
    change.prin();

}
