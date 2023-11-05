#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, to_string_pretty};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct Dog {
        name: String,
        year_born: i32,
        owner: DogOwner,
        #[serde(rename = "dog_breed")]
        breed: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct DogOwner {
        first_name: String,
        last_name: String,
    }
    #[test]
    fn test_serde() {
        let owner01 = DogOwner {
            first_name: "TS".to_string(),
            last_name: "ST".to_string(),
        };
        let dog01 = Dog {
            name: "Cheyenne".to_string(),
            year_born: 2021,
            owner: owner01,
            breed: "B".to_string(),
        };
        let dog_ser = to_string_pretty(&dog01);
        if dog_ser.is_ok() {
            println!("{}", dog_ser.ok().unwrap());
        } else {
            println!("{:#?}", dog_ser.err());
        }
    }
    #[test]
    fn test_serde_deserialize() {
        let json_string = r#"
        {
        "name": "Cheyenne",
        "year_born": 2021,
        "owner": {
            "first_name": "TS",
            "last_name": "ST",
        }
        "dog_breed":"B"
        }"#;

        let dog_deser = from_str::<Dog>(json_string);
        if dog_deser.is_ok() {
            println!("{:#?}", dog_deser.ok().unwrap());
        } else {
            println!("{:#?}", dog_deser.err());
        }
    }
}
