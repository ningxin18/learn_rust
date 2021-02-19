struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dog {} leave", self.name);
    }
}

//提前释放
fn main() {
    let a = Dog{name: String::from("wangcai")};
    let b = Dog{name:String::from("doge")};
    drop(a);//正确，通过std::mem::drop显示清理
    println!("++++++++++ end");
}

