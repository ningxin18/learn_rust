//1.trait用于定义与其他类型共享的功能，类似于go语言中的interface
//可以通过trait以抽象的方式定义共享的行为
//可以使用trait bounds指定泛型是任何拥有特定行为的类型

//2.定义trait
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

//3.实现trait
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetInformation for Student {
    fn get_name(&self) -> &String{
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl GetInformation for Teacher {
    fn get_name(&self) -> &String{
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

//4.默认实现：可以在定义trait的时候提供默认的行为，trait的类型可以使用默认的行为
trait SchoolName {
    fn get_school_name(&self) -> String {
        String::from("s school")
    }
}

impl SchoolName for Student {
    fn get_school_name(&self) -> String {
        String::from("t school")
    }
}

impl SchoolName for Teacher {
    fn get_school_name(&self) -> String {
        String::from("t school")
    }
}

//5.trait作为参数,直接作为参数写入
fn print_info(item: impl GetInformation){
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

//6.trait作为参数,使用trait bound写入，trait_bound语法，指定多个trait bound, 返回trait的类型
//fn print_information<T: GetInformation>(item: T){
//    println!("name = {}", item.get_name());
//    println!("age = {}", item.get_age());
//}

trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

//使用 trait bound 写法一
fn print_information<T: GetName + GetAge>(item: T){
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

//写法二
fn print_information2<T>(item: T)
    where T: GetName + GetAge
{
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

#[derive(Debug)]
pub struct Student2 {
    pub name: String,
    pub age: u32,
}

impl GetName for Student2 {
    fn get_name(&self) -> &String{
        &self.name
    }
}

impl GetAge for Student2 {
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn produce_item_with_age() -> impl GetAge {
    Student2 {
        name: String::from("xiaoming"),
        age: 15,
    }
}

#[derive(Debug)]
pub struct Teacher2 {
    pub name: String,
    pub age: u32,
}

impl GetName for Teacher2 {
    fn get_name(&self) -> &String{
        &self.name
    }
}

impl GetAge for Teacher2 {
    fn get_age(&self) -> u32 {
        self.age
    }
}

//此处会报错，if else不能使用不同类型
//fn item_with_age() -> impl GetAge {
//    let is = true;
//    if is {
//        Strudent2 {
//            name: String::from("xiaoming"),
//            age: 15,
//        }
//    } else {
//        Teacher2 {
//            name: String::from("hulaoshi"),
//            age: 35,
//        }
//    }
//}

//使用trait bound有条件的实现方法
trait GetNameIfelse {
    fn get_name(&self) -> &String;
}
trait GetAgeIfelse {
    fn get_age(&self) -> u32;
}
struct PeopleMatchInfo<T, U> {
    master: T,
    follower: U,
}

impl<T: GetNameIfelse+GetAgeIfelse, U: GetNameIfelse+GetAgeIfelse> PeopleMatchInfo<T, U> {
    fn print_all_info(&self) {
        println!("master name = {}", self.master.get_name());
        println!("master age = {}", self.master.get_age());
        println!("student name = {}", self.follower.get_name());
        println!("student age = {}", self.follower.get_age());
    }
}

struct Master {
    name: String,
    age : u32
}
impl GetNameIfelse for Master {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}

impl GetAgeIfelse for Master {
    fn get_age(&self) -> u32 {
        self.age
    }
}

struct Follower {
    name: String,
    age : u32
}
impl GetNameIfelse for Follower {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}

impl GetAgeIfelse for Follower {
    fn get_age(&self) -> u32 {
        self.age
    }
}


//对任何实现了特定trait的类型有条件的实现trait
trait GetNameIf {
    fn get_name(&self) -> &String;
}
trait PrintName {
    fn print_name(&self);
}
impl<T: GetNameIf> PrintName for T {
    fn print_name(&self) {
        println!("name = {}", self.get_name());
    }
}
struct Worker {
    name: String,
}
impl GetNameIf for Worker {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}

fn main() {
    let s = Student{name:"xiao".to_string(), age:10};
    let t = Teacher{name:"da".to_string(), age: 40, subject:String::from("math")};

    println!("s = {}, {}", s.get_name(), s.get_age());
    println!("t = {}, {}", t.get_name(), t.get_age());

//    print_info(s);
//    print_info(t);

    let s_school_name = s.get_school_name();
    println!("s_school_name = {}", s_school_name);
    let t_school_name = t.get_school_name();
    println!("t_school_name = {}", t_school_name);

    let s2 = Student2{name:"xiao".to_string(), age:10};
    print_information(s2);

    let s3 = produce_item_with_age();
    println!("s3 = {:?}", s3.get_age());

/////////////////////////////////////////////////////////////////////////
    let m = Master{name:"dashi".to_string(), age: 50};
    let f = Follower{name:"pipi".to_string(), age: 25};
    let mf = PeopleMatchInfo{master: m, follower: f};
    mf.print_all_info();
/////////////////////////////////////////////////////////////////////////
    let w = Worker{name:"worker".to_string()};
    w.print_name();
}
