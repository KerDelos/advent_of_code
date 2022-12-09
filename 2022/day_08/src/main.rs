
use std::{vec};

type Pos = (i32, i32);
type Dir = fn(Pos) -> Pos;

fn from_2d_to_1d(pos: Pos, size: usize) -> Option<usize>
{
    let size = i32::try_from(size).unwrap();
    if pos.0 < 0 || pos.0 >= size || pos.1 < 0 || pos.1 >= size {
        return None
    }
    return Some(usize::try_from(pos.0 + pos.1 * size).unwrap());
}

fn from_1d_to_2d(pos: usize, size:usize) -> Option<(i32,i32)>{
    if pos > size * size {
        return None;
    }
    let y = i32::try_from(pos / size).unwrap();
    let x = i32::try_from(pos).unwrap() - y * i32::try_from(size).unwrap();
    return Some((x,y));
}

fn get_tree(pos: Pos, forest: &Vec<u32>, size: usize) -> Option<u32>{
    if let Some(idx) = from_2d_to_1d(pos, size){
        return Some(forest[idx]);
    }
    return None;
}

fn compute_visibility(direction: Dir, starting_positions : Vec<Pos>, forest: &Vec<u32>, size: usize, visibilities: &mut Vec<bool>)
{
    for mut pos in starting_positions {
        let mut max_height = get_tree(pos, &forest, size).unwrap();
        visibilities[from_2d_to_1d(pos, size).unwrap()] = true;
        loop {
            pos = direction(pos);
            match get_tree(pos, forest, size){
                Some(tree_height) => {
                    if tree_height > max_height {
                        max_height = tree_height;
                        visibilities[from_2d_to_1d(pos, size).unwrap()] = true;
                    }
                }
                _ => break,
            }
        }
    }
}

fn compute_scenic_score(posisition: Pos, directions: &Vec<Dir>, forest: &Vec<u32>, size: usize) -> i32{
    let tree_height = get_tree(posisition, forest, size).unwrap();
    let mut score = 1;
    for d in directions{
        let mut pos = posisition;
        let mut nb_trees = 0;
        loop{
            pos = d(pos);
            match get_tree(pos, forest, size){
                Some(other_tree_height) => {
                    nb_trees += 1;
                    if tree_height <= other_tree_height {
                        break;
                    }
                }
                _ => break,
            }
        }
        score *= nb_trees;
    }
    score
}

fn main() {
    let content = std::fs::read_to_string("src/input_1.txt").expect("can't read file");

    let forest_size = content.lines().next().unwrap().chars().count();
    let forest = content.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();

    let north :Dir = |pos :(i32,i32)| (pos.0, pos.1 - 1);
    let south :Dir = |pos :(i32,i32)|  (pos.0, pos.1 + 1);
    let west :Dir = |pos :(i32,i32)| (pos.0 - 1, pos.1);
    let east :Dir = |pos :(i32,i32)| (pos.0 + 1, pos.1);
    let all_dirs = vec![north,south,west,east];

    let north_edge :Vec<(i32,i32)> = (0..forest_size).step_by(1).map(|idx| (i32::try_from(idx).unwrap(), 0)).collect();
    let south_edge :Vec<(i32,i32)> = (0..forest_size).step_by(1).map(|idx| (i32::try_from(idx).unwrap(), i32::try_from(forest_size).unwrap()-1)).collect();
    let west_edge :Vec<(i32,i32)> = (0..forest_size).step_by(1).map(|idx| (0, i32::try_from(idx).unwrap())).collect();
    let east_edge :Vec<(i32,i32)> = (0..forest_size).step_by(1).map(|idx| (i32::try_from(forest_size).unwrap()-1, i32::try_from(idx).unwrap())).collect();

    println!("The forest size is {}",forest_size);
    let mut visibilities = vec![false; forest.len()];
    compute_visibility(south, north_edge, &forest, forest_size, &mut visibilities);
    compute_visibility(north, south_edge, &forest, forest_size, &mut visibilities);
    compute_visibility(east, west_edge, &forest, forest_size, &mut visibilities);
    compute_visibility(west, east_edge, &forest, forest_size, &mut visibilities);
    let nb_trees = visibilities.iter().filter(|v| **v).count();
    println!("From the edges of the forest, you can see {:?} trees", nb_trees);

    let res = forest.iter().enumerate().map(|(i,_)| compute_scenic_score(from_1d_to_2d(i, forest_size).unwrap(), &all_dirs, &forest, forest_size)).max().unwrap();
    println!("max scenic score found is {}", res);
}

#[allow(dead_code)]
fn print_vec_in_2d<T: std::fmt::Display>(vector: &Vec<T>, size: usize)
{
    for y in 0..size{
        for x in 0..size{
            let pos = (i32::try_from(x).unwrap(),i32::try_from(y).unwrap());
            print!("{} ",vector[from_2d_to_1d(pos,size).unwrap()]);
        }
        println!("");
    }
}