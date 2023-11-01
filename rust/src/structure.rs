#[cfg(test)]
mod test {

    use std::cell::Cell;

    #[derive(Debug)]
    struct VehicleTuple(String, String, u16);

    #[derive(Debug)]
    enum VehicleColor {
        Silver,
        Blue,
        Red,
        Black,
        White,
        Green,
    }
    #[derive(Debug)]
    struct Vehicle {
        manufacturer: String,
        model: String,
        year: u16,
        color: VehicleColor,
    }

    impl Vehicle {
        fn paint(&mut self, new_color: VehicleColor) {
            self.color = new_color;
        }
        fn create_vehicle() -> Vehicle {
            let v1 = Vehicle {
                manufacturer: "Porsche".to_string(),
                model: "911".to_string(),
                year: 12,
                color: VehicleColor::Silver,
            };
            return v1;
        }
    }

    struct Person<'p> {
        first_name: Cell<&'p str>,
        last_name: String,
        birth_year: u16,
        birth_month: u8,
        visited_India: bool,
        meters_walked: u32,
    }

    impl Person<'static> {
        fn walk_meters(&mut self, meters: u32) {
            self.meters_walked += meters;
        }
    }

    fn new_person() -> Person<'static> {
        let p1 = Person {
            first_name: Cell::from("Shourya"),
            last_name: "Sharma".to_string(),
            birth_year: 22222,
            birth_month: 4,
            visited_India: true,
            meters_walked: 0,
        };
        p1.first_name.set("Hello");
        return p1;
    }

    // fn new_vehicle() -> Vehicle {}

    fn new_vehicle_tuple() -> VehicleTuple {
        return VehicleTuple("Hyundai".to_string(), "Elantra".to_string(), 2015);
    }
    #[test]
    fn test_struct() {
        let mut p1 = new_person();
        p1.walk_meters(20);
        println!(
            "{} {} {} {} {}",
            p1.first_name.get(),
            p1.last_name,
            p1.birth_month,
            p1.birth_year,
            p1.meters_walked
        );
        let mut my_vehicle = Vehicle::create_vehicle();
        my_vehicle.paint(VehicleColor::Black);
        println!("{:?}", my_vehicle);
        println!("{:?}", new_vehicle_tuple());
    }
}
