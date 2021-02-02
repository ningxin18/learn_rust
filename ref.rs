fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); //s2失效

    let str1 = String::from("hello");
    let str2 = &str1;
//    let len = calcute_legth(&str1);
    let len = calcute_legth(str2);
    println!("len = {}", len);
    println!("str1 = {}", str1);

    let mut ss1 = String::from("hello");
    //modify_s(&ss1); //应用不能修改，报错
    modify_s(&mut ss1); //借用 ：&mut
    println!("ss1 = {}", ss1);

//    let ms = &mut ss1;
//    modify_s(ms);
//    println!("ms = {}", ms);

    let mut rr = String::from("hello");
    let r1 = &rr;
    let r2 = &rr;
    let r3 = &mut s1;
    println!("{},{},{}", r1, r2, r3);
}

fn takes_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

//引用: 用法&，让我创建一个指向值得应用，但是并不拥有它，所以
//当引用离开其值指向的作用域后并不会被丢弃
fn calcute_legth(s: &String) -> usize {
    s.len()
}

fn modify_s(s: &mut String) {
    s.push_str(" , world");
}