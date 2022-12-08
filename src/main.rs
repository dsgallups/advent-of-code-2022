mod helper;
use std::collections::HashSet;

fn main() {
    let mut grid: Vec<Vec<i8>> = Vec::new();
    
    let file:String = helper::file_to_string("src/input-ex");

    
    file.lines().for_each(|line| {


        let vals = line
            .split("")
            .filter(|c| !c.is_empty())
            .map(|c| c.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        grid.push(vals);


    });

    println!("{:?}", grid);

    let mut v_trees: HashSet<(usize, usize)> = HashSet::new();

    /* PART 1

    //from left

    for i in 0..grid.len() {

        let mut tallest_tree: i8 = -1;
        for j in 0..grid[i].len() {
            println!("(i, j) = ({}, {})", i, j);
            if grid[i][j] > tallest_tree {
                println!("inserted!");
                v_trees.insert((i,j));
                tallest_tree = grid[i][j];
            }
        }
    }

    //from right
    for j in 0..grid.len() {

        let mut tallest_tree: i8 = -1;
        for i in (0..grid[j].len()).rev() {
            //println!("(i, j) = ({}, {})", i, j);
            if grid[i][j] > tallest_tree {
                //println!("inserted!");
                v_trees.insert((i,j));
                tallest_tree = grid[i][j];
            }
        }
    }

    //from top
    for j in 0..grid.len() {

        let mut tallest_tree: i8 = -1;
        for i in 0..grid[j].len() {
            //println!("(i, j) = ({}, {})", i, j);
            if grid[i][j] > tallest_tree {
                //println!("inserted!");
                v_trees.insert((i,j));
                tallest_tree = grid[i][j];
            }
        }
    }


    //from bottom
    for i in 0..grid.len() {
        let mut tallest_tree: i8 = -1;
        for j in (0..grid[i].len()).rev() {
            //println!("(i, j) = ({}, {})", i, j);
            if grid[i][j] > tallest_tree {
                //println!("inserted!");
                v_trees.insert((i,j));
                tallest_tree = grid[i][j];
            }
        }
    }
    println!("{:?}", v_trees);
    println!("{:?}", v_trees.len());

    */


    //Analyze from top bottom left right

}