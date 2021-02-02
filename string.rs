fn main() {
//    1.创建一个空String
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);

//    2.通过字面值创建一个String
    let s1 = String::from("init some thing");
    println!("s1 = {}", s1);
    let s1 = "init some thing".to_string(); //后面定义的变量重名就会把前面的隐藏掉

    let mut s2 = String::from("hello");
    s2.push_str(", world");
    let s22 = " !".to_string();
    s2.push_str(&s22);
    println!("s2 = {}", s2);
    println!("s22 = {}", s22); //s22仍可以使用，没有拿走所有权

    let mut s2 = String::from("tea");
    s2.push('m'); //只能添加一个字符，单引号，

    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    let s3 = s1 + &s2;  //+相当于一个方法，s1不能再用，s2是引用，引用不会获取所有权
//    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    let s4 = String::from("four");
    let s5 = String::from("five");
    let s6 = String::from("six");
    let s7 = format!("{}-{}-{}", s4, s5, s6); //fomart!和println!类似
    println!("s7 = {}", s7);

    let s8 = String::from("hello"); //5
//    let s81 = s8[0]; //不能被索引
    println!("s8.len = {}", s8.len());
    let s9 = String::from("你好");
    println!("s9.len = {}", s9.len()); //6  utf8编码

    let hello = "你好";
    let h5 = &hello[0..3]; //取小标,注意边界
    println!("h5 = {}", h5);
//    let h6 = &hello[0..3]; //取小标
//    println!("h6 = {}", h6);

    //chars
    for c in s8.chars(){
        println!("c = {}", c);
    }

    //bytes
    for b in s8.bytes() {
        println!("b = {}", b);
    }
}