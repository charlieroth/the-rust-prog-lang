pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> Result<(), String> {
        if self.h > other.h && self.w > other.w {
            Ok(())
        } else {
            Err(String::from("the rectangle passed in is too big"))
        }
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
    fn can_hold() -> Result<(), String> {
        let r1 = Rectangle { h: 10, w: 10 };
        let r2 = Rectangle { h: 3, w: 3 };
        return r1.can_hold(&r2);
    }
    
    #[test]
    fn cannot_hold() -> Result<(), String> {
        let r1 = Rectangle { h: 10, w: 10 };
        let r2 = Rectangle { h: 3, w: 3 };
        return r2.can_hold(&r1);
    }
}
