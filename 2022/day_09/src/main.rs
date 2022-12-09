use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);
type Dir = fn(Pos) -> Pos;

fn tail_knot(front :Pos, back:Pos) -> Pos {
    if (front.0 - back.0).abs() <= 1 && (front.1 - back.1).abs() <= 1 {
        return back; //back knot is close enough
    }
    let mut delta = (front.0 - back.0, front.1 - back.1);
    delta = (delta.0.max(-1).min(1),delta.1.max(-1).min(1));
    return (back.0 + delta.0, back.1 + delta.1);
}

fn main() {
    let content = std::fs::read_to_string("src/input_1.txt").expect("can't read file");

    let dirs = HashMap::from([
        ("U", (|pos :Pos| (pos.0, pos.1 - 1)) as Dir),
        ("D", (|pos :Pos| (pos.0, pos.1 + 1)) as Dir),
        ("L", (|pos :Pos| (pos.0 - 1, pos.1)) as Dir),
        ("R", (|pos :Pos| (pos.0 + 1, pos.1)) as Dir)
    ]);

    let instructions = content.lines().filter_map(
        |l|  
        match l.splitn(2, ' ').collect::<Vec<_>>()[..]{
            [d, length] => Some((dirs[d],length.parse::<i32>().unwrap())),
            _ => None,
    });

    let mut head_pos = (0,0);
    let mut tail_pos = (0,0);
    let mut visited :HashSet<Pos> = HashSet::new();
    visited.insert(tail_pos);
    for (dir,length) in instructions{
        for _ in 0..length{
            head_pos = dir(head_pos);
            tail_pos = tail_knot(head_pos, tail_pos);
            visited.insert(tail_pos);
        }

        //println!("{:?} {}", dir, length);
        //println!("h: {:?}; t:{:?}", head_pos, tail_pos);
    }

    println!("The tail visited {} positions.", visited.len());
}
