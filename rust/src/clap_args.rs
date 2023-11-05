use clap::{command, Arg, Command};

fn _test_clap() {
    let _match_result = command!()
        .about("This application register people")
        .subcommand(
            Command::new("register-person")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname", "firstname"])
                        .required(true)
                        .help("This argument takes the person's first name"), // .conflicts_with("firstname"),
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname", "lastname"])
                        .required(true)
                        .help("The argument takes the person last name"),
                ),
        )
        .subcommand(
            Command::new("register-pet").arg(
                Arg::new("pet-name")
                    .long("pet-name")
                    .short('n')
                    .required(true),
            ),
        );
    // .group(
    //     ArgGroup::new("Person register")
    //         .arg("firstname")
    //         .arg("lastname"),
    // )
    //     // .group(ArgGroup::new("Dog Register").arg("pet-name"))
    //     .arg(
    //         Arg::new("fluffy")
    //             .long("fluffy")
    //             .help("Is the person wearing a fluffy coat or not"),
    //     )
    //     .get_matches();

    // // println!(
    // //     "Fluffy: {}",
    // //     match_result.get_one::<String>("fluffy").unwrap()
    // // );
    // let pet_args = match_result.subcommand_matches("register_pet");
    // println!(
    //     "Does pet name exist? {}",
    //     pet_args.unwrap().contains_id("pet-name")
    // );
}
