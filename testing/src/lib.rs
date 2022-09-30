pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.h > other.h && self.w > other.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn can_hold() {
        let r1 = Rectangle { h: 10, w: 10 };
        let r2 = Rectangle { h: 3, w: 3 };
        assert_eq!(r1.can_hold(&r2), true);
    }
    
    #[test]
    fn cannot_hold() {
        let r1 = Rectangle { h: 10, w: 10 };
        let r2 = Rectangle { h: 3, w: 3 };
        assert_eq!(r2.can_hold(&r1), false);
    }
}
