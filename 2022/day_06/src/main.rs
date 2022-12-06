use itertools::Itertools;

fn main() {
    let content = std::fs::read_to_string("src/input_1.txt").expect("can't read file");

    let start_of_packet = content.chars().tuple_windows::<(_,_,_,_)>().enumerate()
    .find(|v| vec!(v.1.0,v.1.1,v.1.2, v.1.3).iter().unique().count() == 4)
    .map(|v| v.0).unwrap() + 4;

    let start_of_message = content.chars().collect::<Vec<_>>().windows(14).enumerate()
    .find(|(_,chrs)| chrs.iter().unique().count() == 14).unwrap().0 + 14;

    println!("start-of-packet marker detected at char {}",start_of_packet);
    println!("start-of-message marker detected at char {}",start_of_message);
}
