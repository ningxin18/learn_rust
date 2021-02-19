//1、迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。
//2、创建一个迭代器：迭代器是惰性的，意思是在调用方法使用迭代器之前，它都不会有任何效果。
//3.每个迭代器都实现了iterator trait, iterator trait定义在标准库中

//trait Iterator {
//    type Item;
//    fn next(mut self) -> Option<Self::Item> ; //type Item 和 Self::Item 这种用法叫做定义 trait 的关联类型。
//}

//next 方法：迭代器通过 next 方法来消费一个项。
//Item 类型将是迭代器返回元素的类型，next 方法是 Iterator 实现者被要求定义的唯一方法，next 一次返回一个元素，当迭代器结束，则返回 None。


//自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

//实现自定义迭代器
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter(); // 到目前为止，不会对v1产生任何影响
//    for val in v1_iter {
//        println!("val = {}", val);
//    }

//    获取元素
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    } else {
        println!("End");
    }

//    迭代可变引用，可以调用 iter_mut
    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 3;
    }
    let it = v2.iter();
    //打印：3，2，3
    for val in it {
        println!("{}", val);
    }

//    消费适配器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("total = {}", total);

//    迭代适配器
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("v2 = {:?}", v2);

    let v1 = vec![1, 2, 3, 4];
    let v2: Vec<_> = v1.into_iter().filter(|x| *x > 3).collect();
    println!("v2 = {:?}", v2);

//自定义迭代器
    let mut counter = Counter::new();
    for i in (0..6) {
        if let Some(v) = counter.next() {
            println!("i = {}, v = {}", i, v);
        } else {
            println!("i = {}, at end", i);
            break;
        }
    }
}
