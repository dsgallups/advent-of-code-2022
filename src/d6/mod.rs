mod helper;
mod d6;

fn main() {
    let file:String = helper::file_to_string("src/input-raw");

    let s = file.trim().split("").collect::<Vec<&str>>();
    let mut count = 0;
    let mut buf: Vec<&str> = Vec::new();

    for c in &s {
        for i in 0..buf.len() {
            //println!("at {}", &i);
            if c.eq(&buf[i]) {
                //from the buffer from 0 to the index
                for _ in 0..i+1 {
                    buf.remove(0);
                }
                break;
            }

            //println!("buf[0] = {}", &buf[0]);

            //if i == 0 && buf[0].eq("b") {
              //  println!("this is")
            //}
        }

        if count != 0 {
            buf.push(c);
        }
        println!("Char is: {}", c);
        count += 1;
        if buf.len() >= 14 || count-1 == s.len() {
            println!("result: {}. {}", &count-1, buf.len());
            let newthing = buf.join("");
            println!("buf = {}", &newthing);
            break;
        }
    }

}