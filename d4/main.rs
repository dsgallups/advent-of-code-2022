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

#[allow(dead_code)]
fn string_to_strings<'a>(s: &'a str, delimeter: &'a str) -> Vec<&'a str> {
    let split = s.split(delimeter);
    let result: Vec<&str> = split.collect::<Vec<&str>>();

    return result.clone();

}

fn main() {
    let file = file_to_string("src/input");
    /*
    let ranges: Vec<Vec<u32>> = file
        .split(',')
        .map(|s| {
            s.trim()
                .split('-')
                .map(|s| {
                    println!("{}", &s);
                    s.parse::<u32>().unwrap()
                })
                .collect::<Vec<u32>>()
        })
        //.take(2)
        .collect();

        [[[2, 4],[6, 8]],]
    */

    let ranges: _ = file
        .split('\n')
        .map(|s| {
            s
                .trim()
                .split(',')
                .map(|s| {
                    s
                        .split('-')
                        .map(|s| {
                            s.parse::<u32>().unwrap()
                        })
                        .collect::<Vec<u32>>()
                    
                })
                .collect::<Vec<_>>()

        })
        .collect::<Vec<_>>();

    //println!("hi, {}", ranges[0][1][1]);
    let mut sum: u32 = 0;

    for i in 0..ranges.len() {

        let fl_b = ranges[i][0][0];
        let fu_b = ranges[i][0][1];
        let sl_b = ranges[i][1][0];
        let su_b = ranges[i][1][1];

        let f_len = fu_b - fl_b;
        let s_len = su_b - sl_b;


        //so we get the length of both
        //if the first one is larger, let's do the following
        //get the lower bound of the larger one. 
        //get the lower bound of the upper one
        //if the upper bound of the larger one is greater than or equal to the
        //lower bound of the smaller one, they overlap

        if fl_b < sl_b {
            if fu_b >= sl_b {
                println!("FL_B < SL_B, TRUE: {} {} {} {}", fl_b, fu_b, sl_b, su_b);
                sum += 1
            }
        } else {
            if su_b >= fl_b {
                println!("FL_B > SL_B, TRUE: {} {} {} {}", fl_b, fu_b, sl_b, su_b);
                sum += 1
            }
        }

        /* P1 code
        println!("testing range {}", i);
        if f_len == s_len {
            println!("ranges are equal!");
            if fl_b == sl_b && fu_b == su_b {
                println!("TRUE: {} {} {} {}", fl_b, fu_b, sl_b, su_b);
                sum += 1;
            }
        } else if f_len > s_len {
            println!("first range is bigger!");
            if fl_b <= sl_b && fu_b >= su_b {
                println!("TRUE: {} {} {} {}", fl_b, fu_b, sl_b, su_b);
                sum += 1;
            }
        } else if s_len > f_len {
            println!("second range is bigger!");
            if sl_b <= fl_b && su_b >= fu_b {
                println!("TRUE: {} {} {} {}", fl_b, fu_b, sl_b, su_b);
                sum += 1;
            }
        }
        */

        /*for j in 0..ranges[i].len() {
            for k in 0..ranges[i][j].len() {
                println!("This one is {}", ranges[i][j][k]);
            }
        }*/
    }

    println!("The sum is: {}", sum);

    //ranges[0][1]

}
