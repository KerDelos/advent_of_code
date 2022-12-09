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

fn move_rope(rope: &mut Vec<Pos>, dir: Dir){
    rope[0] = dir(rope[0]);
    for i in 1..rope.len(){
        rope[i] = tail_knot(rope[i-1], rope[i]);
    }
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

    let mut small_rope :Vec<Pos>= vec![(0,0); 2];
    let mut visited_by_small_rope :HashSet<Pos> = HashSet::new();
    visited_by_small_rope.insert((0,0));
    for (dir,length) in instructions.clone(){
        for _ in 0..length{
            move_rope(&mut small_rope, dir);
            visited_by_small_rope.insert(small_rope.last().unwrap().clone());
        }

        //println!("{:?} {}", dir, length);
        //println!("h: {:?}; t:{:?}", head_pos, tail_pos);
    }

    println!("The small rope tail visited {} positions.", visited_by_small_rope.len());
    //assert_eq!(visited.len(), 6037);

    let mut large_rope :Vec<Pos>= vec![(0,0); 10];
    let mut visited_by_large_rope :HashSet<Pos> = HashSet::new();
    visited_by_large_rope.insert((0,0));
    for (dir,length) in instructions{
        for _ in 0..length{
            move_rope(&mut large_rope, dir);
            visited_by_large_rope.insert(large_rope.last().unwrap().clone());
        }
    }

    println!("The large rope tail visited {} positions.", visited_by_large_rope.len());
}
