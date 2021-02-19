//! My Crate lib
//! 'mylib' is a collection of utilites to make calculation more convenient
/// Add one to a number
/// #Example
/// ```
/// let five = 5;
/// assert_eq!(6, mylib::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
