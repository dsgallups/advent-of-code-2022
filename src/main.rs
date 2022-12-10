mod helper;
use std::{collections::HashSet, hash::Hash};

fn add_check_cycle(cycle: &mut i32, reg: i32) -> i32 {

    *cycle += 1;


    if (*cycle - 20) % 40 == 0 {
        return *cycle * reg;
    }

    0
}
fn main() {

    
    let file:String = helper::file_to_string("src/input-raw");

    //create Hashset of points t has visited
    let mut cycle: i32 = 0;
    let mut reg: i32 = 1;
    
    let mut sum = 0;

    file.lines().for_each(|line| {
        let command = line.split(" ").collect::<Vec<&str>>();

        if command[0].eq("noop") {
            sum += add_check_cycle(&mut cycle, reg);
        } else if command[0].eq("addx") {
            sum += add_check_cycle(&mut cycle, reg);

            sum += add_check_cycle(&mut cycle, reg);
            reg += command[1].parse::<i32>().unwrap();
        }
        //println!("CYCLE: {}, REG: {}", cycle, reg);
    });
    
    println!("SUM: {}", sum);
    const CRT_HIGH: u8 = 6;
    const CRT_WIDTH: u8 = 40;
    /*file.lines().for_each(|line| {



        let vals = line
            .split("")
            .filter(|c| !c.is_empty())
            .map(|c| c.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        grid.push(vals);


    });*/



}