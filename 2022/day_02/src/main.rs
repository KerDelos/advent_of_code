use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Sign{
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Outcome{
    Lost = 0,
    Draw = 3,
    Won = 6,
}

fn str_to_sign(s: &str) -> Sign {
    match s {
        "A" | "X" => Sign::Rock,
        "B" | "Y" => Sign::Paper,
        "C" | "Z" => Sign::Scissors,
        _ => panic!("unknown str : {}", s),
    }
}

fn str_to_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Lost,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Won,
        _ => panic!("unknown str : {}", s),
    }
}

fn round_outcome_as_points(you: Sign, other: Sign) -> i32 {
    if you == other {
        return Outcome::Draw as i32
    }
    else if you as u8 - 1 == (other as u8 -1 +1) % 3 {
        return Outcome::Won as i32
    }
    else{
        return Outcome::Lost as i32
    }
}

fn round_sign_as_points(other: Sign, outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Draw => other as i32,
        Outcome::Won => ((other as i32 -1 +1) % 3) + 1,
        Outcome::Lost => (other as i32 -1 -1).rem_euclid(3) + 1,
    }
}

fn main() {
    let file_path = "src/input_1.txt";
    let content = fs::read_to_string(file_path).expect("should be able to read the file");
    let mut part1_sum :i32 = 0;
    let mut part2_sum :i32 = 0;
    for line in content.split('\n') {
        let signs = line.trim().split(" ").collect::<Vec<&str>>();
        if let [other, you] = &signs[..] {
            part1_sum += round_outcome_as_points(str_to_sign(you), str_to_sign(other)) + str_to_sign(you) as i32;
            part2_sum += round_sign_as_points(str_to_sign(other), str_to_outcome(you)) + str_to_outcome(you) as i32;
        };
    }
    println!("solution for part one {}", part1_sum);
    println!("solution for part two {}", part2_sum);
}
