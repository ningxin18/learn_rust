//解引用裸指针
//不可变和可变的，分别写作*const T, *mut T
//1）允许忽略借用规则，可以同时拥有不可变和可变的指针，或是多个指向相同位置的可变指针
//2）不保证指向的内存是有效的
//3）允许为空
//4）不能实现自动清理
// 创建不可变和可变裸指针，可以在安全代码中创建裸指针，只是不能在不安全块之外解引用裸指针。
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize;
    let _r = address as *const i32;

    //使用只能在unsafe里面
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}



