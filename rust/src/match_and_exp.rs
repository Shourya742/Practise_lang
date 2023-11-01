#[cfg(test)]
mod test {

    #[test]
    fn test_match() {
        let myage = 32;
        let y: u8 = 5;
        match myage {
            1..=35 if y == 5 => println!("Oye Yess and arm guard is {}", y),
            1..=35 if y != 5 => println!("Oye Yess and arm guard not {}", y),
            1..=35 => println!("Oye Yess and no arm guard"),
            _ => println!("Don't know"),
        }

        let car_manufacturer = "Porsche";

        match car_manufacturer {
            "Hyundai" => {
                println!("Hyundai it is!")
            }
            "Porsche" => {
                println!("Processing Porsche !!")
            }
            _ => {
                println!("Manufacturer is not supported by this program")
            }
        }

        let prices = [500, 900, 1200];

        match prices[0..=1] {
            [500, 900] => println!("Perfect"),
            [500, _] => println!("Ok"),
            [_, 900] => println!("Ok"),
            _ => println!("Nope"),
        }
    }
}
