use gui::{Screen, Button, SelectBox};

fn main() {
    let s = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
            Box::new(SelectBox {
                width: 50,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("MayBe"),
                ],
            }),
        ],
    };
    s.run();
}

/*
1、trait 对象动态分发
（1）在上一节的 trait 对象例子中，对泛型类型使用 trait bound 编译器进行的方式是单态化处理，单态化的代码进行的是静态分发（就是说编译器在编译的时候就知道调用了什么方法）。
（2）使用 trait 对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于 trait 对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，Rust 在运行时使用 trait 对象中的指针来知晓需要调用哪个方法。

2、trait 对象要求对象安全
只有 对象安全（object safe）的 trait 才可以组成 trait 对象。trait 的方法满足以下两条要求才是对象安全的：

返回值类型不为 Self
方法没有任何泛型类型参数
*/