#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Msg {
    Hello{id: i32},
}

fn main() {
//    1.匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("none"),
    };

//    2.匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("50"),
        Some(y) => println!("value = {}", y),
        _ => println!("others"),
    };
    println!("x = {:?}, y = {:?}", x, y); //x = Some(5), y = 10

//    3.多个模式
    let z = 1;
    match z {
        1|2 => println!("one or two"), //|表示或
        3 => println!("three"),
        _ => println!("none"),
    };

//    4.通过..匹配
    let z1 = 1;
    match z1 {
        1..=5 => println!("one to five"), //|表示或
        _ => println!("none"),
    };

    let z2 = 'c';
    match z2 {
        'a'..='j' => println!("1"),
        'k'..='z' => println!("2"),
        _ => println!("none"),
    }

//    解构并分解值
//    解构元组、结构体、枚举、引用
    let p = Point{x: 1, y: 0};
    let Point{x: a, y:b} = p; //变量a和变量b匹配x和y
    assert_eq!(1, a);
    assert_eq!(0, b);

    match p {
        Point{x, y:0} => println!("x axis"),
        Point{x:0, y} => println!("y axis"),
        Point{x, y} => println!("others"),
    };

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("quit");
        },
        Message::Move{x, y} => {
          println!("move, x: {}, y: {}", x, y);
        },
        Message::Write(text) => println!("write msg: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("color, r: {}, g: {}, b: {}", r, g, b);
        },
    };

    let msg2 = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg2 {
        Message2::Quit => {
            println!("quit");
        },
        Message2::Move{x, y} => {
            println!("move, x: {}, y: {}", x, y);
        },
        Message2::Write(text) => println!("write msg: {}", text),
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("color, r: {}, g: {}, b: {}", r, g, b);
        },
        Message2::ChangeColor(Color::Hsv(r, g, b)) => {
            println!("color, r: {}, g: {}, b: {}", r, g, b);
        },
    };

//    解构结构体和元组
    let ((a,b), Point{x, y}) = ((1, 2), Point{x: 3, y: 4});
    println!("a: {}, b: {}, x: {}, y: {}", a, b, x, y);

//    忽略模式中的值
    foo(1, 2);

    let numbers = (1,2,3,4);
    match numbers {
        (one, _, three, _) => {
            println!("one: {}, three: {}", one, three);
        },
    }

//    变量名前面加_，表示忽略，发挥在编译时发警告
    let _x = 5;
    let _y = 5;
    let s = Some(String::from("hello"));
//    if let Some(c) = s {
//        println!("found a string");
//    }
    if let Some(_) = s {
        println!("found a string");
    }
    println!("s = {:?}", s); //不能再使用s， value borrowed here after partial move
//    println!("c = {:?}", c);

    let nums = (1,2,3,4,5,6,7);
    match nums {
        (first, .., last) => {
            println!("first: {}, last: {}", first, last);
        },
    };

//    匹配守卫提供额外的条件
//    匹配守卫是一个指定于match分支模式之后的if条件，必须满足才能选择此分支
    let num = Some(4);
    let y = 10;
    match num {
        Some(x) if x == y => println!("yes"),
        Some(x) => println!("x:{}", x),
        None => (),
    }

    let x = 4;
    let y = false;
    match x {
        4|5|6 if y => println!("1"),
        _ => println!("2"),
    }

//    @运算符允许我们在创建一个存放值的变量的同时，测试这个变量的值是否匹配模式
    let msg1 = Msg::Hello {id: 5};
    match msg1 {
        Msg::Hello {id: id_va @ 3..=7} => {
            println!("id_va = {}", id_va);
        }
        Msg::Hello {id:  10..=20} => {
            println!("10-20");
        }
        Msg::Hello {id} => {
            println!("id: {}", id);
        }
    }
}

fn foo(_: i32, y: i32) {
    println!("y = {}", y);
}
