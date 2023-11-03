#[cfg(test)]
mod test {
    use std::collections::HashMap;
    #[test]
    fn test_hashmap_basic() {
        let mut stock_list: HashMap<String, f32> = HashMap::<String, f32>::new();
        println!("{}", stock_list.len());
        println!("{}", stock_list.is_empty());
        stock_list.insert("NVDA".to_string(), 478.52);
        stock_list.insert("APPL".to_string(), 232.92);
        stock_list.insert("AMSC".to_string(), 50.78);
        stock_list.insert("APPL".to_string(), 233.43);
        stock_list.entry("META".to_string()).or_insert(346.34);
        println!("{:#?}", stock_list);
        stock_list.remove(&("APPL".to_string()));
        println!("{:#?}", stock_list);

        for (ticker, value) in stock_list {
            println!("{} is trading at {}", ticker, value);
        }
    }
}
