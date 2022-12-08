mod helper;
use std::collections::HashSet;





fn get_scenic_score(g: &Vec<Vec<i8>>, ci: usize, cj: usize) -> u32 {

    //search upwards
    let t_height = g[ci][cj];

    let mut upwards: u32 = 0;
    for i in (0..ci).rev() {

        upwards += 1;

        if g[i][cj] >= t_height {
            break;
        }

    }

    //search leftwards
    let mut leftwards: u32 = 0;
    for j in (0..cj).rev() {
        leftwards += 1;

        if g[ci][j] >= t_height {
            break;
        }
    }

    //search rightwards
    let mut rightwards: u32 = 0;
    for j in (cj + 1)..g[ci].len() {
        rightwards += 1;

        if g[ci][j] >= t_height {
            break;
        }
    }

    //search downards
    let mut downwards: u32 = 0;
    for i in (ci + 1)..g.len() {
        
        downwards += 1;
        if g[i][cj] >= t_height {
            break;
        }
    }


    upwards * leftwards * rightwards * downwards
}

fn main() {
    let mut grid: Vec<Vec<i8>> = Vec::new();
    
    let file:String = helper::file_to_string("src/input-raw");

    
    file.lines().for_each(|line| {


        let vals = line
            .split("")
            .filter(|c| !c.is_empty())
            .map(|c| c.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        grid.push(vals);


    });

    println!("{:?}", grid);

    //let mut v_trees: HashSet<(usize, usize)> = HashSet::new();

    let mut max_scenic_score: u32 = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let scenic_score = get_scenic_score(&grid, i, j);
            
            max_scenic_score = if scenic_score > max_scenic_score { scenic_score } else { max_scenic_score };
        }
    }

    println!("Max Score: {}", max_scenic_score);

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