// Simply return a string
pub fn greeting() -> String {
    let message = String::from("Hello World");
    return message;
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