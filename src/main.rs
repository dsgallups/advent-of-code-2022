use std::thread::current;

use helper::string_to_strings;

mod helper;

const UNDET_REF: usize = 9999999;

struct Directory <'a> {
    name: &'a str,
    items: Vec<usize>,
    sup: usize
}
impl<'a> Directory<'a> {
    pub fn new(name: &'a str, sup: usize) -> Self {
        Directory {
            name: name,
            items: Vec::new(),
            sup: sup
        }
    }
    pub fn push(&mut self, item: usize) {
        self.items.push(item);
    }
    pub fn set_sup(&mut self, sup: usize) {
        self.sup = sup
    }
    pub fn get_sup(&self) -> usize {
        self.sup
    }
    pub fn get_items(&self) -> &Vec<usize> {
        &self.items
    }

    pub fn find(&self, arr: &Vec<ItemType>, name: &str) -> usize {
        

        for i in &self.items {

            match &arr[*i] {
                ItemType::Directory(item) => {
                    if item.get_name().eq(name) {
                        return *i
                    }
                },
                ItemType::File(item) => {
                    if item.get_name().eq(name) {
                        return *i
                    }
                },
                _ => {
                    panic!("Invalid")
                }
            }
            
        }

        UNDET_REF
    }

    pub fn get_name(&self) -> &str {
        self.name
    }
}
struct File<'a> {
    name: &'a str,
    size: u32
}
impl<'a> File<'a> {
    pub fn new(name: &'a str, size: u32) -> Self {
        File {
            name: name,
            size: size
        }
    }
    pub fn get_name(&self) -> &str {
        self.name
    }
    pub fn get_size(&self) -> u32 {
        self.size
    }
}
enum ItemType<'a> {
    Empty,
    Directory(Box<Directory<'a>>),
    File(Box<File<'a>>)
}

/*fn push_item_to_dir<'a> (t: &'a mut ItemType<'a>, item: ItemType<'a>) {

    match t {

        ItemType::Directory(dir) => {
            dir.push(item);
        },
        _ => panic!("Not a directory")

    }
}*/

fn get_size(root_dir: &Vec<ItemType>, current_item: &ItemType, cur_item_i: usize, mut dir_sizes: &mut Vec<u32>) -> u32 {

    match current_item {
        ItemType::Directory(dir) => {
            let items = dir.get_items();
            let mut sum: u32 = 0;
            for i in items {
                sum += get_size(root_dir, &root_dir[*i], *i, &mut dir_sizes);
            }
            //let new_vec = (cur_item_i, sum);
            dir_sizes.push(sum);

            return sum;
        },
        ItemType::File(file) => {
            return file.get_size();
        },
        _ => {
            panic!("Empty Itemtype passed");
        }
    }

}

fn find_dir_size_sum(dir: &Vec<ItemType>, dir_size: u32) -> u32 {

    //start with the root
    let root = &dir[0];
    if let ItemType::Directory(ref direc) = root {
        //now we get the dir object. 

        let items = direc.get_items();
        let mut dir_sizes: Vec<u32> = Vec::new();

        for i in items {
            //println!("hello {}", i);

            let dir_size = get_size(dir, &dir[*i], *i, &mut dir_sizes);
            //println!("{} size: {}", i, dir_size);

        }

        println!("{:?}", dir_sizes);
        /*
        let mut sum: u32 = 0;
        for d in dir_sizes {
            if d.1 <= dir_size {
                sum += d.1;
            }
        }*/
        let mut sum: u32 = 0;

        dir_sizes.sort();

        for i in 0..dir_sizes.len() {
            
        }

        return sum;
    }
    0
}

fn main() {
    let file:String = helper::file_to_string("src/input-raw");

    //let s = file.trim().split("").collect::<Vec<&str>>();

    let mut all: Vec<ItemType> = Vec::new();
    
    let commands = string_to_strings(&file, "\n");

    let mut binding = ItemType::Empty;

    let mut root = ItemType::Directory(Box::new(Directory::new(
        "/",
        UNDET_REF
    )));

    all.push(root);

    let mut cur_dir_i = all.len() - 1;
    //let cur_dir = &all[0];
    let mut i = 1;
    loop {
        let mut output_break = false;
        let line_output = commands[i].split(" ").collect::<Vec<&str>>();
        println!("------------------------------------------------");
        println!("i = {}", i);
        println!("Command: {}", commands[i]);
        println!("cur_dir_i = {}", &cur_dir_i);
        /*if i >= 3 {
            if let ItemType::Directory(ref my_dir) = all[1] {
                println!("A's sup = {} ", my_dir.get_sup());
            }
        }*/
        
        if line_output[0].eq("$") {
            let command = line_output[1];
            if command.eq("ls") {
                let mut j = i + 1;
                loop {
                    println!("------------------------------------------------");
                    println!("j = {}", j);
                    /*if j >= 3 {
                        if let ItemType::Directory(ref my_dir) = all[1] {
                            println!("A's sup = {} ", my_dir.get_sup());
                        }
                    }*/
                    
                    //println!("Command is: {}", )
                    if j == commands.len() || commands[j].contains("$") {
                        i = j;
                        output_break = true;
                        println!("Breaking!!");
                        break;
                    }
                    let output = commands[j].split(" ").collect::<Vec<&str>>();
                    println!("Command: {}", commands[j]);
                    println!("cur_dir_i = {}", &cur_dir_i);

                    if output[0].eq("dir") {
                        //current_directorypush((output[1], &Vec::new()));
                        /*current_directory.push(ItemType::Directory(
                                Box::new(
                                    Directory::new(output[1])
                                )
                            )
                        );*/
                        let new_dir = ItemType::Directory(
                            Box::new(
                                Directory::new(
                                    output[1],
                                    cur_dir_i
                                )
                            )
                        );
                        //push new_dir to all array, then grab indice
                        all.push(new_dir);

                        //get position of array
                        //cur_dir_i = all.len() - 1;
                        let old_i = cur_dir_i;
                        let new_i = all.len() - 1;
                        println!("new entry = {}", &new_i);

                        if let ItemType::Directory(ref mut my_dir) = all[cur_dir_i] {
                            my_dir.push(new_i);
                        }

                        //cur_dir_i = new_i;
                        
                        //give new directory a super
                        if let ItemType::Directory(ref mut my_dir) = all[new_i] {
                            my_dir.set_sup(old_i);

                            println!("super directory = {}", &my_dir.get_sup());
                        }
                        
                        
                    } else {
                    

                        let new_file = ItemType::File(
                            Box::new(
                                File::new(
                                    output[1],
                                    output[0].parse::<u32>().unwrap()
                                )
                            )
                        );

                        all.push(new_file);
                        let new_i = all.len() - 1;
                        println!("new entry = {}", &new_i);

                        if let ItemType::Directory(ref mut my_dir) = all[cur_dir_i] {
                            my_dir.push(new_i);
                        }
                    
                    }
                    
                    j += 1;
                } //inner loop
            } else if line_output[1].eq("cd") {
                //println!("COMMAND IS CD---------");
                //check if it's a name or a ..
                if line_output[2].eq("..") {

                    
                    let mut sup_i = UNDET_REF;
                    
                    if let ItemType::Directory(ref my_dir) = all[cur_dir_i] {
                        sup_i = my_dir.get_sup()
                    }

                    cur_dir_i = sup_i;

                    println!("new cur_dir_i = {}", &cur_dir_i);

                } else {
                    //println!("IN HERE-----------------------------");
                    let mut next_i = UNDET_REF;
                    //find the directory where the next dir is stored
                    if let ItemType::Directory(ref my_dir) = &all[cur_dir_i] {
                        next_i = my_dir.find(&all, line_output[2]);
                    }

                    cur_dir_i = next_i;
                    println!("new cur_dir_i = {}", &cur_dir_i);
                }
            } else {
                println!("line_output = {}", line_output[0]);
                panic!("INVALID COMMAND");
            }
        } //if command is $

        if i == commands.len() {
            break;
        }
        if output_break == true {
            output_break = false;
        } else {
            i += 1;
        }
        println!("end outer loop. directory complete.");
    } //outer loop

    //now just find the biggest directory
    let res = find_dir_size_sum(&mut all, 100000);
    println!("Sum: {}", &res);
}