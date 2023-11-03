#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn test_hashset() {
        let mut planet_list = HashSet::from(["Mercury", "Venu", "Earth"]);
        let planet_list_more = HashSet::from(["Earth", "Mars", "Jupiter"]);
        let planet_diff = planet_list.difference(&planet_list_more);
        let planet_sym_diff = planet_list.symmetric_difference(&planet_list_more);
        // for planet in planet_list {
        //     println!("Thanks for adding {}", planet);
        // }

        planet_list.insert("Saturn");
        planet_list.insert("Uranus");
        planet_list.insert("Neptune");
        planet_list.insert("Saturn");
        planet_list.insert("Uranus");
        planet_list.insert("Neptune");
        planet_list.insert("Saturn");
        planet_list.insert("Uranus");
        planet_list.insert("Neptune");

        for planet in planet_list {
            println!("Thanks for adding {}", planet);
        }
    }
}
