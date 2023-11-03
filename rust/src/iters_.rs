#[cfg(test)]
mod test {
    #[test]
    fn test_rust_iterators() {
        let fruit_list = vec!["Strawberry", "Blueberry", "Mango", "Orange", "Apple"];
        let nut_list = vec!["Walnut", "Almonds", "Brazil Nuts", "Pecans"];
        let mut fruit_iter = fruit_list.iter();
        // for fruit in fruit_iter {
        //     println!("{}", fruit);
        // }
        fruit_iter.next();
        let item01 = fruit_iter.next();
        println!("First item int iterator is: {}", item01.unwrap());

        let aggregate_foods = fruit_list.iter().chain(&nut_list);

        let all_foods: Vec<&&str> = aggregate_foods.clone().collect();

        for food in aggregate_foods {
            println!("Eating {}", food);
        }

        let fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));
        let new_fruits = fruit_list_strings.map(|mut e| {
            e.push_str(" Fruit");
            return e;
        });
        new_fruits.clone().for_each(|e| println!("{}", e));
        println!("{}", new_fruits.clone().last().unwrap());

        let mut stepby = new_fruits.clone().step_by(2);
        println!("Step: {}", stepby.next().unwrap());
        println!("Step: {}", stepby.next().unwrap());
        println!("Step: {}", stepby.next().unwrap());

        let first_name = vec!["Carol", "Ross", "Rachel", "Joe"];
        let first_name_string = first_name.iter().map(|e| String::from(*e));
        let last_name = vec!["Jones", "Sullivan", "Tanner", "Redman"];
        let last_name_string = last_name.iter().map(|e| String::from(*e));

        let full_name = first_name_string.zip(last_name_string);
        full_name.clone().take(2).for_each(|e| {
            println!("{} {}", e.0, e.1);
        });

        for (index, value) in full_name.clone().skip(1).enumerate() {
            println!("Index: {} value: {} {}", index, value.0, value.1);
        }

        let foods = vec![("potatoes", 10), ("strawberries", 25), ("burgers", 12)];
        let food_quantity = foods.clone().iter().fold(0, |mut a, e| a + e.1);
        println!("{}", food_quantity);

        foods.iter().peekable().next();
        let food_iter = foods.iter();
        let mut mypeekable = food_iter.peekable();
        mypeekable.next();
        let food = mypeekable.peek();
        println!("Peeking at {}", food.unwrap().0);
    }
}
