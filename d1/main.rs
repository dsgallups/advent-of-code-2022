
use std::io::prelude::*;
use std::fs;

fn main() {

    //let most_calories = get_most_calories("src/input");
    println!("Most Calories: {}", get_most_calories("src/input"));

    let elf_sums: Vec<i32> = get_ordered_sum_of_all_elves("src/input");

    //grab the first three and dip
    let mut final_sum = 0;
    for n in 0..3 {
        println!("N = {}", n);
        final_sum += elf_sums[n];
    }

    println!("Sum of Three Elves with most cals: {}", final_sum);


}

fn file_to_string(path: &str) -> String {
    let mut file: fs::File = fs::File::open(path)
        .expect("File at path");

    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("File as a string");

    data
}

fn get_ordered_sum_of_all_elves(path: &str) -> Vec<i32> {
    let data = file_to_string(path);

    let elves: std::str::Split<&str> = data.split("\n\n");

    

    let mut elf_total_calories: Vec<i32> = Vec::new();

    for elf in elves {
        let elf_items = elf.split("\n");

        let mut elf_cumulative_cals = 0;
        for item in elf_items {
            let cals = item.parse::<i32>().unwrap();
            elf_cumulative_cals += cals
        }
        elf_total_calories.push(elf_cumulative_cals);
    }

    //Sort elf_total_calories highest to lowest
    elf_total_calories.sort_by(|a, b| b.cmp(a));
    elf_total_calories
    

}

fn get_most_calories(path: &str) -> i32 {
    
    let data = file_to_string(path);

    
    //now split data into individual pieces
    let elf_loads = data.split("\n\n");

    let mut most_calories = 0;

    for elf in elf_loads {

        let elf_items = elf.split("\n");

        let mut elf_cumulative_cals = 0;
        for item in elf_items {
            let cals = item.parse::<i32>().unwrap();
            elf_cumulative_cals += cals
        }

        most_calories = if most_calories < elf_cumulative_cals {elf_cumulative_cals} else {most_calories};
    }

    most_calories
}