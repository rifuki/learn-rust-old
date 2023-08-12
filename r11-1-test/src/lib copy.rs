struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

mod tests {
    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = super::Rectangle {
            length: 20,
            width: 10
        };
        let rect2 = super::Rectangle {
            length: 20,
            width: 5
        };

        assert!(!rect1.can_hold(&rect2));
    }
}