mod modA {
    #[derive(Debug)]
    pub struct A {
        pub number: i32,
        name: String,
    }
    impl A {
        pub fn newA() -> A {
            A {
                number: 1,
                name: String::from("A"),
            }
        }
        pub fn print_a(&self) {
            println!("number: {}, name: {}", self.number, self.name);
        }
    }

    pub mod modB {
        pub fn print_B() {
            println!("B");
        }
        pub mod modC {
            pub fn print_C() {
                println!("C");
                super::print_B();
            }
        }
    }
}

use modA::A;
fn main() {
    let a = modA::A::newA();
    a.print_a();

    let aa = A::newA();
    aa.print_a();

    let number = a.number;
//    let name = a.name; //私有，不能直接调用，可以使用模块调用

    modA::modB::modC::print_C();

}


