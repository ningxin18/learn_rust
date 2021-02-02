//use mylib::factory::produce_refrigerator;
//use mylib::factory::produce_washing_machine as A;
use mylib::factory::*;

//模块控制作用域和私有性，默认属性为私有
fn main() {
    mylib::factory::produce_refrigerator::produce_re(); //绝对路径
    produce_refrigerator::produce_re(); //使用use
//    A::produce_washing_maching();
    produce_washing_machine::produce_washing_maching();
}


