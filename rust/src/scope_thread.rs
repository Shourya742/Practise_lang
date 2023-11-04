#[cfg(test)]
mod test {
    use std::thread::scope;

    struct Person {
        first_name: String,
    }
    #[test]
    fn test_scoped_thread() {
        let age = 34;
        let person01 = Person {
            first_name: String::from("Shourya"),
        };
        let print_age = || {
            println!("Your age is {age}");
            println!("Your name is: {}", &person01.first_name)
        };
        std::thread::scope(|scope| {
            scope.spawn(print_age);
        });
        // let _result = thread::spawn(print_age).join();
        println!("Your age is {age}");
        println!("Your name is: {}", person01.first_name);
        println!("Finished printing age");
    }
}
