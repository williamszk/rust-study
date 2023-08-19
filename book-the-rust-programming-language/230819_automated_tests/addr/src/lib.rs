pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // self.length > other.length && self.width > other.width
        self.length < other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration(){
        assert_eq!(2+2, 4);
    }

    // #[test]
    // fn another(){
    //     panic!("Make test fail!");
    // }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{length:8,width:7};
        let smaller = Rectangle{length:5,width:1};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle{length:8,width:7};
        let smaller = Rectangle{length:5,width:1};

        assert!(!smaller.can_hold(&larger));
    }
}
