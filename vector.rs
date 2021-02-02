fn main() {
    //1.创建空的vector: Vec<T>
    let mut v: Vec<i32> = Vec::new(); //必须mut，空的不可变没有意义

    //2.创建包含初始值的vector
    let v = vec![1,2,3];

    //3.丢弃vector //自动回收
    {
        let v1 = vec![1, 2, 3];
    }

    //4.读取元素, 直接索引(越界会panic)和get方法
    let one: &i32 = &v[0];
    println!("one = {}", one);
    println!("one = {}", *one);

    //推荐的方法，越界会判断，走默认，不会panic，便于程序容错处理
    match v.get(1) {
        Some(value) => println!("value = {}", value),
        _ => println!("None"),
    }
    //5.更新元素
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    //6.遍历
//    不可变遍历
    for i in &v2 {
        println!("i = {}", i)
    }

//    可变遍历
    for i in &mut v2 {
        *i += 1;
        println!("i = {}", i);
    }

   //7.使用枚举
    enum Context {
       Text(String),
       Float(f32),
       Int(i32),
   };

    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.1),
    ];

    //8.注意
//    let mut v = vec![1, 2, 3, 4, 5];
//    let first = &v[0]; //创建不可变引用
//    v.push(6); //可变之后，不能使用变之前的引用，否则报错
//    println!("first = {}", first);
}