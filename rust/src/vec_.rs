#[cfg(test)]
mod test {
    #[test]
    fn test_vec_int() {
        let mut my_ints: Vec<i32> = Vec::new();
        my_ints.push(30);
        for x in 4..100 {
            my_ints.push(x * 10);
        }
        println!("Size of Vec: {:?}", my_ints.len());
        println!("Capacity of Vec: {:?}", my_ints.capacity());
        println!("{:?}", my_ints);

        println!("First item in Vec is: {:?}", &(&my_ints).as_slice()[0..5]);
        println!("First element is: {:?}", my_ints.get(10).unwrap());
    }

    #[test]
    fn test_vec_string() {
        let first_names = vec!["Xy", "xY", "xy", "XY"];

        for first_name in first_names {
            println!("Processing {} ...", first_name);
        }
    }

    #[derive(Debug, Clone)]
    struct Car {
        manufacturer: String,
        model: String,
    }

    #[test]
    pub fn test_vec_car() {
        let mut car_list = vec![
            Car {
                manufacturer: "Porsche".to_string(),
                model: "Panamera".to_string()
            };
            100
        ];
        car_list.reserve(1000);
        let mut car_lot2: Vec<Car> = vec![];

        for _ in 1..=100u8 {
            car_lot2.push(Car {
                manufacturer: "Hyundai".to_string(),
                model: "Sonata".to_string(),
            });
        }
        car_list.append(&mut car_lot2);
        car_list.retain(|e: &Car| {
            if e.manufacturer == "Porsche" {
                return true;
            } else {
                return false;
            }
        });
        println!("{:?}", car_list);
    }
}
