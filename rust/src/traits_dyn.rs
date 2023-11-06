#[cfg(test)]
mod test {
    struct Dog {}
    struct Antelope {}
    struct Bear {}

    trait AnimalEating {
        fn eat_food(&self);
    }
    trait AnimalSound {
        fn make_sound(&self);
    }

    trait Animal: AnimalEating + AnimalSound {}

    impl AnimalEating for Dog {
        fn eat_food(&self) {
            println!("Dog is eating dog food");
        }
    }

    impl AnimalEating for Antelope {
        fn eat_food(&self) {
            println!("Antelope is eating natural deset plants");
        }
    }

    impl AnimalEating for Bear {
        fn eat_food(&self) {
            println!("Bear is eating some other Animal");
        }
    }

    impl AnimalSound for Dog {
        fn make_sound(&self) {
            println!("Dog is barking!");
        }
    }

    impl AnimalSound for Antelope {
        fn make_sound(&self) {
            println!("Antelope is bleating");
        }
    }

    impl AnimalSound for Bear {
        fn make_sound(&self) {
            println!("Bear is roaring!");
        }
    }

    impl Animal for Antelope {}
    impl Animal for Dog {}
    impl Animal for Bear {}

    fn make_some_noise(a: &dyn AnimalSound) {
        a.make_sound()
    }

    fn eat_some_food(a: &dyn AnimalEating) {
        a.eat_food()
    }

    fn get_animal() -> Box<dyn Animal> {
        let bear = Bear {};
        return Box::from(bear);
    }

    #[test]
    fn test_trait_type() {
        let dog01: &dyn AnimalSound = &Dog {};
        let antelope01: &dyn AnimalSound = &Antelope {};
        make_some_noise(dog01);
        make_some_noise(antelope01);

        let dog01: &dyn AnimalEating = &Dog {};
        let antelope01: &dyn AnimalEating = &Antelope {};
        eat_some_food(dog01);
        eat_some_food(antelope01);

        let bear01 = get_animal();
        bear01.eat_food();
        bear01.make_sound();
    }
}
