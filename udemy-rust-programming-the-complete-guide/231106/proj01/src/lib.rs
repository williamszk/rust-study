pub fn add(left: usize, right: usize) -> usize {
    left + right
}


fn _simple_add()->bool{
    if 2+2 == 4 {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    // #[ignore]
    #[should_panic]
    fn test02(){
        panic!("It failed");
    }

    #[test]
    fn test03(){
        assert!(_simple_add());
    }
    
    #[test]
    fn test04() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }
}
