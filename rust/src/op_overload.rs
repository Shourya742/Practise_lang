#[cfg(test)]
mod test {
    use std::ops::Add;
    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String,
    }
    #[derive(Debug)]
    struct Marriage {
        husband: Person,
        wife: Person,
        location: String,
        date: chrono::NaiveDate,
    }

    impl Add for Person {
        type Output = Marriage;
        fn add(self, rhs: Self) -> Self::Output {
            Marriage {
                husband: self,
                wife: rhs,
                location: "XY".to_string(),
                date: chrono::offset::Local::now().date_naive(),
            }
        }
    }

    struct GroceryItem {
        name: String,
        price: f32,
    }

    struct GroceryBill {
        items: Vec<GroceryItem>,
        tax_rate: f32,
    }

    impl GroceryBill {
        fn calculate_total(&self) -> f32 {
            let mut items_total = self.items.iter().fold(0f32, |a, i| return a + i.price);
            let tax_value = items_total * self.tax_rate;
            return items_total + tax_value;
        }
    }

    impl Add<GroceryItem> for GroceryBill {
        type Output = GroceryBill;
        fn add(self, rhs: GroceryItem) -> Self::Output {
            let mut bill = self;
            bill.items.push(rhs);
            return bill;
        }
    }

    #[test]
    fn test_() {
        let person1 = Person {
            first_name: "X".to_string(),
            last_name: "X".to_string(),
        };
        let person2 = Person {
            first_name: "Y".to_string(),
            last_name: "Y".to_string(),
        };

        let marriage = person1 + person2;
        println!("{:#?}", marriage);

        let mut new_bil = GroceryBill {
            items: Vec::<GroceryItem>::new(),
            tax_rate: 0.027,
        };
        let carrots = GroceryItem {
            name: "Bag of Carrots 1 pound".to_string(),
            price: 2.2,
        };
        let cheese = GroceryItem {
            name: "Cottage Cheese 12oz".to_string(),
            price: 1.2,
        };
        new_bil = new_bil + carrots + cheese;
        let total = new_bil.calculate_total();
        println!("The total of your grocery bill is: {total}");
    }
}
