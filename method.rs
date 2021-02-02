//打印
#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

impl Dog {
    fn get_name(&self) -> &str {
        //self --> java this 指针
        &(self.name[..])
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn show() {
        println!("oh wo");
    }
}

fn main() {
    let dog = Dog {
        name: String::from("jack"),
        weight: 100.0,
        height: 70.1,
    };
    println!("dog = {:#?}", dog);
    println!("name = {}", dog.get_name());
    println!("height = {}", dog.get_height());
    println!("weight = {}", dog.get_weight());

    Dog::show();
}