pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,  //trait对象，使用dyn关键字
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button, width = {}, height = {}, label = {}",
                 self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selectBox, width = {}, height = {}, options = {:?}",
                 self.width, self.height, self.options);
    }
}

////复习
//pub struct Screen<T: Draw> {
//    pub components: Vec<T>,
//}
//
//impl<T> Screen<T>
//    where T: Draw {
//    pub fn run(&self) {
//        for component in self.components.iter() {
//            component.draw();
//        }
//    }
//}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
