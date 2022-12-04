use regex::Regex;

fn do_range_overlap_completely(r1 : (i32, i32), r2 : (i32,i32)) -> bool {
    return ((r1.0..=r1.1).contains(&r2.0) && (r1.0..=r1.1).contains(&r2.1))
    || ((r2.0..=r2.1).contains(&r1.0) && (r2.0..=r2.1).contains(&r1.1));
}

fn do_range_overlap(r1 : (i32, i32), r2 : (i32,i32)) -> bool {
    return ((r1.0..=r1.1).contains(&r2.0) || (r1.0..=r1.1).contains(&r2.1))
    || ((r2.0..=r2.1).contains(&r1.0) || (r2.0..=r2.1).contains(&r1.1));
}

fn main() {
    let content = std::fs::read_to_string("src/input_1.txt").expect("can't read file");
    
    let reg = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();

    let parsed_pairs = reg.captures_iter(&content)
    .map(|m| m.iter().collect::<Vec<_>>()[1..=4]
                        .iter().map(|e| e.expect("").as_str().parse::<i32>().expect(""))
                        .collect::<Vec<_>>())
    .map(|v| ((v[0],v[1]), (v[2],v[3])))
    .collect::<Vec<_>>();
    
    let pb1 = parsed_pairs.iter().filter(|pair| do_range_overlap_completely(pair.0, pair.1)).count();
    let pb2 = parsed_pairs.iter().filter(|pair| do_range_overlap(pair.0, pair.1)).count();


    println!("There are {} assignments completely overlaping", pb1);
    println!("There are {} assignments patially overlaping", pb2);


}
