use std::io::prelude::*;
use std::fs;

fn file_to_string(path: &str) -> String {
    let mut file: fs::File = fs::File::open(path)
        .expect("File at path");

    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("File as a string");

    data
}


fn string_to_strings<'a>(s: &'a str, delimeter: &'a str) -> Vec<&'a str> {
    let split = s.split(delimeter);
    let result: Vec<&str> = split.collect::<Vec<&str>>();

    return result.clone();

}

fn print_crates(crates: &Vec<Vec<&str>>, step: &usize) -> () {

    println!("-----------------------------------------------------------");
    println!("Crates for step {}", step);
    for i in 0..crates.len() {
        print!("Crate {}    --", &i);

        for j in 0..crates[i].len() {
            print!(" {} ", crates[i][j]);
        }
        print!("\n");
    }
    println!("-----------------------------------------------------------");
}

fn print_instructions(ins: &Vec<usize>) {
    println!("-----------------------------------------------------------");
    for i in 0..ins.len() {
        println!("{}", ins[i]);
    }
}

fn get_output(crates: &Vec<Vec<&str>>) -> String {

    println!("output is here");
    let mut ret = String::from("").to_owned();

    for i in 0..crates.len() {
        ret.push_str(crates[i][crates[i].len()-1])
    }

    ret
}

fn main() {
    let f = file_to_string("src/input");

    let lines = string_to_strings(&f, "\n");
    //println!("{} {} {}", lines[0].len(), lines[1].len(), lines[6].len());



    let crate_line_len = lines[0].len();
    let mut stacks: Vec<Vec<&str>> = Vec::new();

    //alright im actually doing this the easier way. I'm just gonna iterate through
    //the lines until a line is not the same size as the first one. then im gonna march it backwards
    //and add things to it like a stack

    let mut crate_height = 0;
    for line in &lines {
        if line.len() == crate_line_len {
            crate_height += 1;
        } else {
            break;
        }
    }

    //with this info, lets take a look at something
    //prints that last line
    //println!("line: {}", &lines[crate_height - 2]);

    //create our vector
    /*
        The number of boxes can be determined by the first line's length
        length = 4 * n - 1
        so number of boxes is
        n = (length + 1) / 4
    */
    let vector_length = (crate_line_len + 1) / 4;
    //println!("Vector Length: {}", vector_length);

    
    for _ in 0..vector_length {
        stacks.push(Vec::new());
    }

    //println!("Stacks Length: {}", stacks.len());

    //build our crates from bottom to top. It starts at
    //crate_height - 2 and ends at 0.
    let mut i = crate_height - 2;

    /*
        so there will be chars at places
        length + 2 will be the size of our array;
            2   6   10
            0   1   2

            6 - 2 / 4 = 1
            10 - 2 / 4 = 2
        index 0 has some character
        index 12 has some character
        so 1-11 have the values for our example
     */
    loop {
        let boxes: Vec<&str> = lines[i].split("").collect();

        //first char at 2
        let mut j = 2;

        while j < boxes.len() {
            if boxes[j].eq(" ") {
                //println!("ain't nothin here: {} {}", &i, &j);
            } else {
                //get the indice to insert into
                let indice = (j - 2) / 4;
                stacks[indice].push(boxes[j]);
            }
            j += 4;
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }

    //println!("Stack 1: {} {}", stacks[1][0], stacks[1][1]);

    //now we can do fun things
    //take our crate_height variable and add 1 to get our first instruction
    let instruction_start = crate_height + 1;
    //println!("First Instruction: {}", lines[instruction_start]);

    //parse each instruction into three values
    //[[amt: u32, crate_no: usize, ]]

    fn get_usize_from_char (line: &str, val: usize) -> usize {
        let vec_line = line
            .split(" ")
            .collect::<Vec<&str>>();

        //println!("Char to parse: {}, {} {}", vec_line[offset + 1], line, offset);

        //this is because of the +1 rule. still don't know what starts and ends these strings...

        //check if the digit afterwards is 
        return vec_line[val].parse::<usize>().unwrap();

    }

    #[allow(dead_code)]
    fn get_instruction(ins: &Vec<Vec<usize>>, position: usize) -> usize {
        return ins[ins.len()-1][position];
    }
    let mut instructions: Vec<Vec<usize>> = Vec::new();
    for i in instruction_start..lines.len() {

        let mut line_instructions: Vec<usize> = Vec::new();

        line_instructions.push(get_usize_from_char(lines[i], 1));
        line_instructions.push(get_usize_from_char(lines[i], 3));
        line_instructions.push(get_usize_from_char(lines[i], 5));


        instructions.push(line_instructions);
        //println!("Offsets = {} {} {}", get_instruction(&instructions, 0), get_instruction(&instructions, 1), get_instruction(&instructions, 2));
        

    }
    
    //now we have our instructions and our crates parsed. Let's freakin do this thing.
    //first let's print out our initial crates
    print_crates(&stacks, &0);
    let mut j: usize = 1;
    for instruction in instructions {

        //print_instructions(&instruction);
        //amount
        let amt = instruction[0];
        let frm = instruction[1] - 1;
        let to = instruction[2] - 1;

        //so, we will do this for X amount of times
        for _ in 0..amt {
            //grab the highest value from the "frm" crate
            /*
                Part 1 CODE
                let val = stacks[frm].pop().unwrap();
                stacks[to].push(val);
            */
        }

        //so we'll have a for loop that will copy from one crate to the other
        //PART 2 CODE
        let mut it = amt;
        loop {
            if it == 0 {
                break;
            }
            let val = stacks[frm][stacks[frm].len() - it];
            stacks[to].push(val);
            it -= 1;
        }

        //then a for loop to pop the crates from the frm crate.
        it = amt;
        loop {
            if it == 0 {
                break;
            }
            stacks[frm].pop();
            it -= 1;
        }


        print_crates(&stacks, &j);
        j += 1;
    }

    println!("The result is: {} ", get_output(&stacks));

}