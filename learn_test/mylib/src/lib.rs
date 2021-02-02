pub mod animal;

#[cfg(test)]
mod tests {
    use crate::animal::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn use_cat() {
//        cat::hello();
        assert_eq!(true, cat::is_cat());
    }

    #[test]
    fn use_dog() {
        assert_eq!(true, dog::is_dog());
    }


}
