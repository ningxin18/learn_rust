fn main() {
    // let中使用if
    let condition = true;
    let x = if condition {
        5
    } else {
        6
    }; //两个条件分支中必须同一类型
    println!("x = {}", x);

    //loop
    let mut counter = 0;
    loop {
        println!("in loop");
        if counter ==  10 {
            break;
        }
        // counter = counter + 1;
        counter += 1;
    }

    let result = loop {
        counter += 1;
        if counter ==  20 {
            break counter*2;
        }
    };
    println!("result = {}", result);

    // while循环
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}", i);

    // for循环
    let arr:[u32; 5] = [1,2,3,4,5];
    for element in arr.iter() {
        println!("element = {}", element);
    }

    for element in &arr {
        println!("element = {}", element);
    }
}