mod to_do;
use to_do::ItemTypes;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use crate::to_do::traits::get::Get;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use std::env;
use std::fs::{File,self};
use std::io::Read;
use serde_json::{Map,json,value::Value};

pub fn read_file(file_name:&str)->Map<String,Value> {
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value =  serde_json::from_str(&data).unwrap();
    let state:Map<String,Value> = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name:&str,state:&mut Map<String,Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write file");
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String,Value> = read_file("./state.json");
    println!("Before operation: {:?}",state);
    state.insert(title.to_string(), json!(status));
    println!("After opeartion: {:?}",state);
    write_to_file("./state.json",&mut state);
}
