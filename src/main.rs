mod helper;
use std::{collections::HashSet, hash::Hash};


const GRID_WIDTH: i32 = 6;
const GRID_HEIGHT: i32 = 5;

fn get_t_pos(h: &(i32, i32), t: &(i32, i32)) -> (i32, i32) {

    let i_dis = (h.0 - t.0).abs();
    let j_dis = (h.1 - t.1).abs();
    //println!("DISTANCE: ({}, {})", i_dis, j_dis);

    if i_dis <= 1 && j_dis <= 1 {return (t.0, t.1);}
    if i_dis > j_dis {
        if h.0 > t.0 {
            return (h.0-1, h.1);
        } else {
            return (h.0+1, h.1);
        }

    } else if j_dis > i_dis {
        if h.1 < t.1 {
            return (h.0, h.1 + 1);
        } else {
            return (h.0, h.1 - 1);
        }
    } else {
        println!("H: {:?}; T: {:?}", h, t);
        panic!("IMPOSSIBLE");
    }
}
fn main() {

    
    let file:String = helper::file_to_string("src/input-raw");

    //create Hashset of points t has visited
    let mut t_vis: HashSet<(i32, i32)> = HashSet::new();

    //set starting points for h and t
    let mut h = (4, 0);
    let mut t = (4, 0);

    println!("START ---");
    println!("H: {:?}; T: {:?}", h, t);

    file.lines().for_each(|line| {
        let com = line.split(" ").collect::<Vec<&str>>();

        println!("COMMAND: {}", line);

        if com[0].eq("R") {
            for _ in 0..com[1].parse::<u32>().unwrap() {
                h = (h.0, h.1 + 1);
                t = get_t_pos(&h, &t);
                t_vis.insert(t);
                println!("H: {:?}; T: {:?}", h, t);
            }

        } else if com[0].eq("U") {
            for _ in 0..com[1].parse::<u32>().unwrap() {
                h = (h.0 - 1, h.1);
                t = get_t_pos(&h, &t);
                t_vis.insert(t);
                println!("H: {:?}; T: {:?}", h, t);
            }

        } else if com[0].eq("L") {
            for _ in 0..com[1].parse::<u32>().unwrap() {
                h = (h.0, h.1 - 1);
                t = get_t_pos(&h, &t);
                t_vis.insert(t);
                println!("H: {:?}; T: {:?}", h, t);
            }

        } else if com[0].eq("D") {
            for _ in 0..com[1].parse::<u32>().unwrap() {
                h = (h.0 + 1, h.1);
                t = get_t_pos(&h, &t);
                t_vis.insert(t);
                println!("H: {:?}; T: {:?}", h, t);
            }

        } else {
            panic!("Invalid Command!");
        }
        

        /*if h.1 >= GRID_WIDTH || t.1 >= GRID_WIDTH || h.0 >= GRID_HEIGHT || t.0 >= GRID_HEIGHT {
            panic!("Invalid position; H: {:?}; T: {:?}", h, t)
        }*/
    });


    println!("T VISITED:");
    println!("{:?}", &t_vis);
    println!("Length: {}", &t_vis.len());
    
    /*file.lines().for_each(|line| {



        let vals = line
            .split("")
            .filter(|c| !c.is_empty())
            .map(|c| c.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        grid.push(vals);


    });*/



}