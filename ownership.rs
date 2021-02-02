//1. rust通过所有权机制来管理内存，编译器再编译就会根据所有权规则对内存的使用进行检查
//2.堆和栈
//编译的时候数据类型大小是固定的，就是分配在栈上
//编译的时候数据类型大小不固定，就是分配在堆上
//3.作用域:{}

//4.String内存回收

fn main() {
    let x:i32 = 1;
    {
        let y:i32 = 1;
        println!("x = {}", x);
    }
    //    println!("y = {}", y);
    {
        let mut s1 = String::from("hello");
        s1.push_str(" world");
        println!("s1 = {}", s1); //String类型离开作用域的时候会调用drop方法释放内存

        let str1 = String::from("hello");
        let str2 = str1; //str1被借用了
        println!("str2 = {}", str2); //类似C++潜拷贝， move to str2, str1 Invalid

        //clone 深拷贝
        let str3 = str2.clone();
        println!("str2 = {}", str2);
        println!("str3 = {}", str3);
    }

    //copy 栈上拷贝, 堆有指针指向，栈是拷贝两份，copy trait
    let a = 1;
    let b = a;
    println!("a = {}, b = {}", a, b);
    //常用的具有copy trait有：所有的整形，浮点型 布尔值 字符类型 char， 元组

}