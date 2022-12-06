use std::{collections::VecDeque, cmp::Reverse};
use regex::Regex;

fn print_state(state: &Vec<VecDeque<char>>)
{
    let cl = (*state).clone();
    for i in 0..cl.len() {
        print!("{} ",i+1);
        let cl2 = cl[i].clone();
        for cr in cl2 {
            print!("[{}] ", cr);
        }
        println!("");
    }
}


fn main() {
    let content = std::fs::read_to_string("src/input_1.txt").expect("can't read file");
    
    
    if let [state, moves ] = content.split("\r\n\r\n").collect::<Vec<_>>()[0..=1] {
        
        //initializing dock state
        let mut dock: Vec<VecDeque<char>> = Vec::new();
        for line in state.lines() {
            if dock.is_empty() {
                let dock_size = (line.len() + 1) / 4;
                println!("Found a dock of size {}", dock_size);
                for _ in 0..dock_size {
                    dock.push(VecDeque::new());
                }
            }
            line.match_indices(|c: char| c.is_alphabetic()).for_each(|c| dock[c.0/4].push_back(c.1.chars().nth(0).expect("")));  
        }

        //processing moves
        let moves_regex = Regex::new(r"move (?P<number>\d*) from (?P<from>\d*) to (?P<to>\d*)").unwrap();
        
        let mut dock_9000 = dock.clone();
        for mv in moves_regex.captures_iter(moves) {
            print_state(&dock_9000);
            println!("\nmove {} from {} to {}", &mv["number"], &mv["from"],&mv["to"]);
            let from =  mv["from"].parse::<usize>().unwrap() - 1;
            let to =  mv["to"].parse::<usize>().unwrap() - 1;
            for i in 0..(&mv["number"]).parse::<usize>().unwrap() {
                let cr = dock_9000[from].pop_front().unwrap();
                dock_9000[to].push_front(cr);
            }
        }

        //part 2
        let mut dock_9001 = dock.clone();
        for mv in moves_regex.captures_iter(moves) {
            print_state(&dock_9001);
            println!("\nmove {} from {} to {}", &mv["number"], &mv["from"],&mv["to"]);
            let nb = mv["number"].parse::<usize>().unwrap();
            let from =  mv["from"].parse::<usize>().unwrap() - 1;
            let to =  mv["to"].parse::<usize>().unwrap() - 1;
            let remaining = dock_9001[from].split_off(nb);
            let moving_crates = dock_9001[from].clone();
            dock_9001[from] = remaining;
            moving_crates.iter().rev().for_each(|c| dock_9001[to].push_front(*c));
        }

        let solution_9000 = dock_9000.iter().map(|col| match col.front() { Some(c) => *c, None => ' '}).collect::<String>();
        let solution_9001 = dock_9001.iter().map(|col| match col.front() { Some(c) => *c, None => ' '}).collect::<String>();
        println!("Crates on top with the 9000 are {}",solution_9000);
        println!("Crates on top with the 9001 are {}",solution_9001);
        assert_eq!(solution_9000, "FCVRLMVQP");
        assert_eq!(solution_9001, "RWLWGJGFD");
    }
}