use macro_hello::HelloMacro;
use macro_hello_derive::HelloMacro;

#[derive(HelloMacro)]
struct Main;

fn main() {
    Main::hello_macro();

}
