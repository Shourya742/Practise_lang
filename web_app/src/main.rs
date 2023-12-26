use clap::{Arg,App};

fn main() {
    let app = App::new("booking").version("1.0").about("Books in a user").author("Maxwell Flitton");
    let first_name = Arg::new("first name").long("f").takes_value(true).help("first name of user").required(true);
    let second_name = Arg::new("second name").long("l").takes_value(true).help("last name of user").required(true);
    let age = Arg::new("age").long("a").takes_value(true).help("age of the user").required(true);

    let app = app.arg(first_name).arg(second_name).arg(age);
    let matches = app.get_matches();
    let name = matches.value_of("first name").expect("First name is required");
    let surname =  matches.value_of("second name").expect("Second name is required");
    let age = matches.value_of("age").expect("Age is required");
    println!("{:?} {:?} {:?}",name,surname,age);
}