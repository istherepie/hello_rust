static HELLO: &str = "Hello World";

pub fn greeting<'a>() -> &'a str {
    return HELLO
}


#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn test_greeting() {
        let expected = "Hello World";
        let test_value = greeting();

        // Assert
        assert_eq!(expected, test_value);
    }
}