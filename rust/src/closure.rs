struct Person {
    first_name: String,
    last_name: String,
}

fn test_closures() {
    let add = || println!("Returning some text");
    add();
}

fn test_cloures2() {
    let add = |x, y| {
        println!("x: {} y: {}", x, y);
        x + y
    };
    let result: i32 = add(2, 5);

    let print_result = |x| println!("The result is: {}", (result + x));
    print_result(100);

    let mut p1 = Person {
        first_name: "Shourya".to_string(),
        last_name: "Sharma".to_string(),
    };

    let mut change_name = |new_last_name: &str| p1.last_name = new_last_name.to_string();
    change_name("SHarma");
    change_name("Sharma");
    println!("{}", p1.last_name);
}

#[cfg(test)]
mod test {

    #[test]
    fn closure() {
        super::test_closures();
        super::test_cloures2()
    }
}
