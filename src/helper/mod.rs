use std::io::prelude::*;
use std::fs;


pub fn file_to_string(path: &str) -> String {
    let mut file: fs::File = fs::File::open(path)
        .expect("File at path");

    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("File as a string");

    data
}

pub fn string_to_strings<'a>(s: &'a str, delimeter: &'a str) -> Vec<&'a str> {
    let split = s.split(delimeter);
    let result: Vec<&str> = split.collect::<Vec<&str>>();

    return result.clone();

}
