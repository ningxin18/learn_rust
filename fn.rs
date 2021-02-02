//蛇形命名法

fn main() {
 other_fn();

 let a: i32 = -1;
 let b: u32 = 1;
 fn_params(a, b);

 let c: i32 = 1;
 let r: i32 = add(a, c);
 println!("r = {}", r);

 //语句是执行一些操作，但是不返回值得指令
// let y = 1; //语句，不返回值
// let x = (let y = 1); //这样不行

 //表达式会计算一些值
 let y = {
  let x = 1;
//  x + 1; 加;不返回值，报错
  x + 1
 };
 println!("y = {}", y);
}

fn other_fn(){
  println!("function");
}

//函数类型需定义
fn fn_params(a: i32, b: u32) {
 println!("a = {}, b = {}", a, b);
}

fn add(a: i32, b: i32) -> i32 {
 let result = a + b;
// return result;
 result
}



