fn main() {
    // bool
    let is_true: bool = true;
    println!("is_true = {}", is_true);

    let is_false: bool = false;
    println!("is_false = {}", is_false);

    // char 在rust里面，char是32位，字符(unicode编码，占用4个字节)，C++是8位
    let a = 'a';
    println!("a = {}", a);

    let b = '你';
    println!("b = {}", b);

    // 数字类型
    let u8 = 0_u8;
    println!("u8 = {}", u8);

    let u16 = 0_u16;
    let u32 = 0_u32;
    let u64 = 0_u64;
    let u128 = 0_u128;

    // 有符号
    let i8 = -111;
    println!("i8 = {}", i8);
    let i16 = 0_i16;

    //浮点型
    let f1:f32 = 0.0;
    let f2:f64 = 0.0;

    //自适应类型 isize, usize, i有符号，u无符号
    println!("max = {}", usize::max_value());

    //数组 [Type; size], size也是数组类型的一部分
    let arr: [u32; 5] = [1,2,3,4,5];
    println!("arr[0] = {}", arr[0]);

    //元组
    let tup: (i32, f32, char) = (-3, 12.1, 'a');
    let tup = (-3, 12.1, 'a');
    println!("{}", tup.0);
    let (x,y,z) = tup;
    println!("{}", x);
}

fn show(arr:[u32;3]) {
    println!("------------");
    println!("arr[0] = {}", arr[0]);
    for i in &arr {
        println!("{}", i);
    }
    println!("------------");
}

