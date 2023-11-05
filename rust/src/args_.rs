use std::env::args;

pub fn process_args() {
    let myargs: Vec<String> = args().collect();
    println!("{:?}", myargs);

    if myargs.len() != 3 {
        println!("Hey,you didn't specify two arguments.");
        return;
    }
    let name = myargs.get(1).unwrap().into();
    let year_born = myargs.get(2).unwrap().parse::<i32>();

    if year_born.is_err() {
        println!("The specified dog year is invalid");
    }
    let year = year_born.unwrap();
    println!("{name} {}", year);
    let d1 = Dog::new(name, year);
    println!("{:#?}", d1);
    d1.get_details();
}

#[derive(Debug)]
struct Dog {
    name: String,
    year_born: i32,
}

impl Dog {
    fn new(name: String, year_born: i32) -> Self {
        return Dog { name, year_born };
    }

    fn get_details(&self) {
        println!(
            "Dog was born in year {} and its name is {}",
            self.year_born, self.name
        );
    }
}
