use std::collections::HashMap;
fn main() {
//    1.HashMap<K,V>
//    2.创建HashMap
//    2.1常规创建
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

//    2.2 iter创建
    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
//    _占位符，不关注类型, zip
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

//    3.读取
    let k = String::from("Blue");
//    let v = scores.get(&k); //v = 10; 这里返回的是option，有可能为空,
    if let Some(v) = scores.get(&k) {
        println!("v = {}", v);
    }
    let v1 = scores.get(&k); //get 返回的是一个Option
    match v1 {
        Some(value) => println!("v = {}", value),
        None => println!("None"),
    }
//    4.遍历: 会以任意的顺序遍历出来
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }

//    5.更新
    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.insert(String::from("one"), 3); //one更新
    println!("{:?}", ss);

//    键不存在的时候才插入
    let mut ss1 = HashMap::new();
    ss1.insert(String::from("one"), 1);
    ss1.insert(String::from("two"), 2);
    ss1.insert(String::from("three"), 3);
    ss1.entry(String::from("one")).or_insert(3); //one更新
    println!("{:?}", ss1);

//    根据旧值来更新一个值
    let text = "hello world good world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(1);
        *count += 1;
    }
    println("map = {:?}", map);
}