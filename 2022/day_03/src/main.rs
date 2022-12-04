use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn item_priority(item: u8) -> i32{
    let offset = if item < 'a' as u8 { 'A' as u8 - 27 } else { 'a' as u8 - 1 };
    return (item - offset) as i32;
}

fn missplaced_object(rucksack: &str) -> u8 {
    let len = rucksack.len();
    let first_half: HashSet<u8> = HashSet::from_iter(rucksack[..len/2].bytes());
    let second_half: HashSet<u8> = HashSet::from_iter((&rucksack[len/2..].bytes()).clone().into_iter());
    return first_half.intersection(&second_half).next().expect("").clone();
}

fn trio_tag(trio: &[&str]) -> u8 {
    trio.into_iter()
        .map(|rucksack| HashSet::<u8>::from_iter(rucksack.bytes().clone().into_iter()))
        .reduce(|a, b| HashSet::<u8>::from_iter(a.intersection(&b).map(|e| *e).collect::<Vec<u8>>())).expect("")
        .into_iter().next().expect("")
}

fn main() {
    let content = fs::read_to_string("src/input_0.txt")
        .expect("Should have been able to read the file");

    let sum_missplaced_objects: i32 = content.lines()
        .filter(|l| !l.is_empty())
        .map(|rucksack| item_priority(missplaced_object(rucksack)))
        .sum();

    let sum_tags_priority: i32 = content.lines()
        .filter(|l| !l.is_empty()).collect::<Vec<_>>()
        .chunks(3)
        .map(|trio| item_priority(trio_tag(trio)))
        .sum();

    println!("The sum of priotiy of misplaced objects is {}",sum_missplaced_objects);
    println!("The sum of priotiy of tags is {}",sum_tags_priority);

}
