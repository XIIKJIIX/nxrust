pub mod inner {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn minus(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
