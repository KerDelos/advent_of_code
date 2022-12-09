type Pos = (i32, i32);
type Dir = fn(Pos) -> Pos;

fn main() {
    let content = std::fs::read_to_string("src/input_0.txt").expect("can't read file");

    let up :Dir = |pos :Pos| (pos.0, pos.1 - 1);
    let down :Dir = |pos :Pos| (pos.0, pos.1 + 1);
    let left :Dir = |pos :Pos| (pos.0 - 1, pos.1);
    let right :Dir = |pos :Pos| (pos.0 + 1, pos.1);

    let instructions = content.lines().filter_map(
        |l|  
        match l.splitn(2, ' ').collect::<Vec<_>>()[..]{
            [dir, length] => Some((dir,length.parse::<i32>().unwrap())),
            _ => None,
    });

    for (dir,length) in instructions{
        println!("{} {}", dir, length);
    }
}
