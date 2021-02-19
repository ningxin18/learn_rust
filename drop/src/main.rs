//Drop trait 类似于其它语言中的析构函数，当值离开作用域时执行此函数的代码。
//可以为任何类型提供 Drop trait 的实现（但是注意，这里的类型需要用 struct 包含起来，用例子实现 Drop for i32 和 Drop for String 报错）。

struct Dog {
    name: String,
//    count: i32,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dog {} leave", self.name);
//        self.count -= 1;
    }
}

fn main() {
    let a = Dog{name: String::from("wangcai")};
    {
        let b = Dog{name:String::from("doge")};
        println!("++++++++++0");
    }
    println!("++++++++++1");
}
