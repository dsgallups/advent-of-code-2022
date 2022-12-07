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

fn main() {
    let file:String = helper::file_to_string("src/input-ex");

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
    for i in 1..commands.len() {
        let line_output = commands[i].split(" ").collect::<Vec<&str>>();
        if line_output[0].eq("$") {
            let command = line_output[1];
            if command.eq("ls") {
                let mut j = i + 1;
                loop {
                    let output = commands[j].split(" ").collect::<Vec<&str>>();

                    if commands[j].contains("$") || output[0].contains("$") {
                        break;
                    }

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

                        if let ItemType::Directory(ref mut my_dir) = all[cur_dir_i] {
                            my_dir.push(new_i);
                        }

                        cur_dir_i = new_i;

                        //give new directory a super
                        if let ItemType::Directory(ref mut my_dir) = all[cur_dir_i] {
                            my_dir.set_sup(old_i);
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

                        if let ItemType::Directory(ref mut my_dir) = all[cur_dir_i] {
                            my_dir.push(new_i);
                        }
                    
                    }
                    
                    j += 1;
                }
            } else if command.eq("cd") {
                //check if it's a name or a ..
                if line_output[2].eq("..") {
                    let mut sup_i = UNDET_REF;
                    
                    if let ItemType::Directory(ref my_dir) = all[cur_dir_i] {
                        sup_i = my_dir.get_sup()
                    }

                    cur_dir_i = sup_i;

                } else {
                    let mut next_i = UNDET_REF;
                    //find the directory where the next dir is stored
                    if let ItemType::Directory(ref my_dir) = &all[cur_dir_i] {
                        next_i = my_dir.find(&all, line_output[2]);
                    }

                    cur_dir_i = next_i
                }
            } else {
                panic!("INVALID COMMAND");
            }
        }
    }

    //
        

    
    //root.into().
    /*if let ItemType::Directory(ref my_root) = root {
        let name = my_root.get_name();
        println!("Root name: {}", name);

    }

    let current_directory = &mut root;*/
    /*
    for i in 1..commands.len() {
        let line_output = commands[i].split(" ").collect::<Vec<&str>>();
        if line_output[0].eq("$") {
            let command = line_output[1];
            if command.eq("ls") {
                let mut j = i + 1;
                loop {
                    let output = commands[j].split(" ").collect::<Vec<&str>>();

                    if commands[j].contains("$") || output[0].contains("$") {
                        break;
                    }

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
                                    output[1]
                                )
                            )
                        );
                        push_item_to_dir(current_directory, new_dir);
                        
                    } else {
                    

                        let new_file = ItemType::File(
                            Box::new(
                                File::new(
                                    output[1],
                                    output[0].parse::<u32>().unwrap()
                                )
                            )
                        );
                        push_item_to_dir(current_directory, new_file);
                    
                    }
                    
                    j += 1;
                }
            } else if command.eq("cd") {
                //check if it's a name or a ..
                if line_output[2].eq("..") {
                    //current_directory = current_directory.sup;
                } else {
                    

                }

            }
        }
    }*/
    

    /*let mut root_dir = Type {
        name: "/",
        i_type: ItemType::Directory,
        dir: Vec::new(),
        sup: None,
        size: 0
    };
    let current_directory = &mut root_dir;

    for i in 1..commands.len() {
        let line_output = commands[i].split(" ").collect::<Vec<&str>>();
        //println!("{}", line_output[0]);
        if line_output[0].eq("$") {
            //println!("true");

            let command = line_output[1];
            if command.eq("ls") {
                let mut j = i + 1;
                loop {              


                    let output = commands[j].split(" ").collect::<Vec<&str>>();

                    if commands[j].contains("$") || output[0].contains("$") {
                        break;
                    }

                    if output[0].eq("dir") {
                        //current_directorypush((output[1], &Vec::new()));
                        current_directory.dir.push(
                            Type {
                                name: output[1],
                                i_type: ItemType::Directory,
                                dir: Vec::new(),
                                sup: {Some(&mut current_directory)},
                                size: 0
                            }
                        )
                    } else {
                        current_directory.dir.push(
                            Type {
                                name: output[1],
                                i_type: ItemType::File,
                                dir: Vec::new(),
                                sup: Some(current_directory),
                                size: output[0].parse::<u32>().unwrap()
                            }
                        )
                    }
                    
                    j += 1;

                }


            } else if command.eq("cd") {

                //current_directory = 


            } else {
                panic!("INVALID COMMAND!");
            }
        }

    }*/
 
}