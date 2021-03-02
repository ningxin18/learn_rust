#![crate_type = "staticlib"]
#[no_mangle]
pub extern fn foo() {
    println!("use rust");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
