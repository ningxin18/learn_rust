use getaver::AverCollect;
fn main() {
    let mut a = getaver::AverCollect::new();
    a.add(1);
    println!("average value = {}", a.average());

    a.add(2);
    println!("average value = {}", a.average());

    a.add(3);
    println!("average value = {}", a.average());

    a.remove();
    println!("average value = {}", a.average());
}

/*
rust没有继承的概念，可以通过trait来进行行为共享

trait A {
    fn sum() {
        //todo
    }
}

struct AX {

}

impl A for AX {

}
*/