#[cfg(test)]
mod test {

    trait Animal {
        fn make_sound(&self) -> ();
    }

    trait NotDangerous {}
    trait Dangerous {}
    struct Person<PetType: Animal + NotDangerous, PetType2>
    where
        PetType2: Animal + Dangerous,
    {
        pet: PetType,
        pet_d: PetType2,
        first_name: String,
    }
    struct Dog {}
    impl NotDangerous for Dog {}
    impl Animal for Dog {
        fn make_sound(&self) -> () {
            println!("Bark");
        }
    }
    struct Cat {}
    impl NotDangerous for Cat {}
    impl Animal for Cat {
        fn make_sound(&self) -> () {
            println!("Meow")
        }
    }

    struct Bear {}
    impl Animal for Bear {
        fn make_sound(&self) -> () {
            println!("Bear roared")
        }
    }
    impl Dangerous for Bear {}
    struct Tiger {}
    impl Animal for Tiger {
        fn make_sound(&self) -> () {
            println!("Tiger Roared")
        }
    }
    impl Dangerous for Tiger {}

    #[test]
    fn create_person() {
        let pet1 = Dog {};
        let pet2 = Tiger {};
        let p1 = Person {
            first_name: "Hello".to_string(),
            pet: pet1,
            pet_d: pet2,
        };
        p1.pet.make_sound();
        p1.pet_d.make_sound();
    }
}
