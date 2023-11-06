#[cfg(test)]
mod test {

    use std::default::Default;

    #[derive(Debug)]
    struct FirstName(String);
    impl Default for FirstName {
        fn default() -> Self {
            return FirstName("Shourya".to_string());
        }
    }

    #[derive(Debug)]
    struct Person {
        first_name: FirstName,
        last_name: String,
        age: u8,
        location: String,
    }

    impl Default for Person {
        fn default() -> Self {
            Person {
                first_name: FirstName::default(),
                last_name: "Sharma".to_string(),
                age: 1,
                location: "12".to_string(),
            }
        }
    }

    #[test]
    fn test_default_trait() {
        let p01 = Person::default();
        println!("{:#?}", p01);
    }
}
