//1.泛型是具体类型或者其它属性的抽象替代，用于减少代码重复
//2.在函数定义中使用泛型
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//泛型约束，泛型要求满足条件：PartialOrd可以按照顺序比较，Copy具有Copy特征（int, char）
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

//3.在结构体中使用泛型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

//4.在枚举中使用泛型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

//5.在方法中使用泛型
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

impl<T, U> Point2<T, U> {
    fn create_point<V, W>(self, other:Point2<V, W>) -> Point2<T, W> {
        Point2{
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![1,2,3,100];
    let max_number = largest_i32(&number_list);
    println!("max_number = {}", max_number);

    let char_list = vec!['a', 'y', 'b'];
    let max_char = largest_char(&char_list);
    println!("max_char = {}", max_char);

    let max_number = largest(&number_list);
    println!("max_number = {}", max_number);

    let max_char = largest(&char_list);
    println!("max_char = {}", max_char);

    ////////////////////////////////////////////////////////////////////////
    let integer = Point{x: 1, y: 2};
    println!("interger = {:#?}", integer);

    let float = Point{x: 1.1, y: 2.2};
    println!("float = {:?}", float);

    let a = Point2{x:1.1, y: 'a'};
    println!("float = {:#?}", a);

    ////////////////////////////////////////////////////////////////////////
    let p = Point{x:1, y:2};
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());

    let p = Point{x:1.1, y:2.1};
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());

    ////////////////////////////////////////////////////////////////////////
    let p1 = Point2{x: 5, y: 1.1};
    let p2 = Point2{x: "hello", y: 'c'};
    let p3 = p1.create_point(p2);
    println!("p3 = {:#?}", p3)
}

