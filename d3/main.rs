use std::hash::Hash;
use std::io::prelude::*;
use std::fs;
use std::collections::HashMap;


fn file_to_string(path: &str) -> String {
    let mut file: fs::File = fs::File::open(path)
        .expect("File at path");

    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("File as a string");

    data
}

/*fn string_to_strings(i_str: String, delimiter: String) -> Vec<&str> {
    let s_items: Split<&str> = i_str.split(&delimiter);
    let result: Vec<&str> = s_items.collect::<Vec<&str>>();

    result
}*/

fn string_to_strings<'a>(s: &'a str, delimeter: &'a str) -> Vec<&'a str> {
    let split = s.split(delimeter);
    let result: Vec<&str> = split.collect::<Vec<&str>>();

    return result.clone();

}

fn get_priority(b: u8) -> i32 {

    let mut ret: i32 = 0;

    //println!("B = {}", i32::from(b));
    //for uppercase
    if b >= 65u8 && b <= 90u8 {
        //println!("{} is lowercase.", i32::from(b));
        //A is 65
        //Priority is 27,
        //so ret = 65 - 38
        ret += i32::from(b - 38u8);

    } else if b >= 97u8 && b <= 122u8 {
        //println!("{} is uppercase.", i32::from(b));
        ret += i32::from(b - 96u8);
    }
    return ret;
}

fn main() {
    let s_file = file_to_string("src/input");

    let rucksacks = string_to_strings(s_file.as_str(), "\n");


    let mut sum = 0;
    let mut counter = 0;

    let mut character_count: HashMap<u8, i32> = HashMap::new();
    
    for rucksack in rucksacks {
        let r_b = rucksack.as_bytes();
        let len = r_b.len();

        if counter == 3 {
            //reset
            character_count = HashMap::new();
            counter = 0;
        }

        //get unique characters per sack
        let mut used_chars: Vec<u8> = Vec::new();
        for i in 0..len {
            if !used_chars.contains(&r_b[i]) {
                used_chars.push(r_b[i]);
            }
        }
        //create a hashmap that has uses character:count
        for char in used_chars {
            //check if char is already in hashmap
            /*match character_count.get(&char) {
                Some(&count) => 
            }*/
            //character_count.insert(char, )
            character_count.insert(char, 1 + if character_count.contains_key(&char) { character_count[&char] } else { 0 });

            if character_count[&char] >= 3 {
                println!("Found common char: {}", char::from(char));
                sum += get_priority(char);
            }
                
        }
        //if the character's count is 3, grab its priority and return it

        




        counter += 1;



        //println!("length: {}", len);
        /*
            length is 24:
            0-23
            0-11, 12-23
        */
        /*
            PART 1
        
        let mut keep_running = true;
        for i in 0..(len/2) {
            //println!("{:?}", r_b[i]);
            for j in (len/2)..len {
                if r_b[i] == r_b[j] {
                    sum += get_priority(r_b[i]);    
                    keep_running = false;
                    break;
                }
            }
            if keep_running == false {
                break;
            }

        }*/

        //sack_no+=1;
    }
    println!("Sum = {}", sum);

}
