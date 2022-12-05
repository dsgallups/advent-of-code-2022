use std::io::prelude::*;
use core::str::Split;
use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors
}

fn file_to_string(path: &str) -> String {
    let mut file: fs::File = fs::File::open(path)
        .expect("File at path");

    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("File as a string");

    data
}

fn get_action(action: &str) -> Rps {
    match action {
        "A" | "X" => Rps::Rock,
        "B" | "Y" => Rps::Paper,
        "C" | "Z" => Rps::Scissors,
        &_ => panic!("Invalid Value!")
    }
}

fn determine_winner(p_a: Rps, o_a: Rps) -> i32 {
    if p_a == o_a {
        return 3;
    }

    if p_a == Rps::Rock {
        if o_a == Rps::Paper {
            return 0;
        }
        if o_a == Rps::Scissors {
            return 6;
        }
    } else if p_a == Rps::Paper {
        if o_a == Rps::Scissors {
            return 0;
        }
        if o_a == Rps::Rock {
            return 6;
        }
    } else if p_a == Rps::Scissors {
        if o_a == Rps::Rock {
            return 0;
        }
        if o_a == Rps::Paper {
            return 6;
        }
    }

    panic!("No valid winner!");
}

fn get_round_outcome(o_a: Rps, outcome: &str) -> Rps {

    if outcome.eq("Y") {
        return o_a
    }
    if o_a == Rps::Rock {
        //Loss
        if outcome.eq("X") {
            return Rps::Scissors
        }
        
        //Win
        if outcome.eq("Z") {
            return Rps::Paper
        }
    }

    if o_a == Rps::Paper {
        //Loss
        if outcome.eq("X") {
            return Rps::Rock
        }
        
        //Win
        if outcome.eq("Z") {
            return Rps::Scissors
        }
    }

    if o_a == Rps::Scissors {
        //Loss
        if outcome.eq("X") {
            return Rps::Paper
        }
        
        //Win
        if outcome.eq("Z") {
            return Rps::Rock
        }
    }

    panic!("No valid outcome");
}

fn main() {
    
    let input = file_to_string("src/input");

    //let elves: std::str::Split<&str> = data.split("\n\n");
    let str_rounds: Split<&str> = input.split("\n");

    let mut cumulative_score = 0;
    for str_round in str_rounds {
        let split_actions: Split<&str> = str_round.split(" ");
        let actions: Vec<&str> = split_actions.collect::<Vec<&str>>();

        let opponent_action: Rps = get_action(actions[0]);
        let round_outcome: &str = actions[1];

        let player_action: Rps = get_round_outcome(opponent_action, round_outcome);

        let mut score = 0;
        if player_action == Rps::Rock {
            score += 1;
        } else if player_action == Rps::Paper {
            score += 2;
        } else if player_action == Rps::Scissors {
            score += 3;
        }

        if round_outcome.eq("X") {
            score += 0;
        } else if round_outcome.eq("Y") {
            score += 3;
        } else if round_outcome.eq("Z") {
            score += 6;
        }

        //score += determine_winner(player_action, opponent_action);

        cumulative_score += score;

    }

    println!("The total score is: {}", cumulative_score);

}
