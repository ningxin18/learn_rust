fn main() {
    //    1.定义结构体
    struct User {
        name: String,
        count: String,
        nonce: u64,
        active: bool,
    }
//    2.创建结构体实例
    let xiaoming = User {
        name: String::from("xiaoming"),
        count: String::from("100000000"),
        nonce: 100,
        active: true,
    };
    println!("user = {:?}", xiaoming);
    println!("user = {:#?}", xiaoming); //自动换行

//    3.修改结构体字段
    let mut xiaohong = User {
        name: String::from("xiaohong"),
        count: String::from("100000000"),
        nonce: 100,
        active: true,
    };
    xiaohong.nonce = 200;

//    4.参数名字和字段名字同名的简写方法
    let name = String::from("xiaoxiao");
    let count = String::from("188681");
    let nonce = 200;
    let active = false;
    let user1 = User {
        name,
        count,
        nonce,
        active,
    };

// 5.从其他的结构体创建实例
    let user2 = User {
        name: String::from("user2"),
        ..user1
    };
    println!("user2.name = {}", user2.name);
    println!("user2.nonce = {}", user2.nonce)
//    6。元组结构体，字段没有名字；圆括号；
    struct Point(i32, i32);
    let a = Point(1, 2);
    println!("a.0 = {}, a.1 = {}", a.0, a.1)

//    7.没有任何字段的结构体

}