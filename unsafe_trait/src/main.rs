//实现不安全的trait
//（1）当至少有一个方法中包含编译器不能验证的不变量时，该 trait 是不安全的；
//（2）在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe。
//
struct Bar();

unsafe trait Foo {
    fn foo(&self);
}

unsafe impl Foo for Bar{
    fn foo(&self) {
        println!("foo");
    }
}

fn main() {
    let a: Bar = Bar();
    a.foo();
    println!("Hello, world!");
}
