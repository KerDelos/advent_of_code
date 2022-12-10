fn new_cycle(cycle: &mut i32){
    *cycle += 1;
}

fn check_frequency(cycle: & i32, register: &i32, signal_strength : &mut i32){
    if (cycle+20) % 40 == 0{
        println!("cycle is {}", cycle);
        println!("register is {}", register);
        *signal_strength += cycle * register;
        println!("signal is {} = {}",cycle*register, signal_strength);
    }
}

fn main() {
    let content = std::fs::read_to_string("src/input_1.txt").unwrap();

    let mut cycle = 1;
    let mut register = 1;
    let mut signal_strength = 0;
    for line in content.lines(){
        //println!("{:?}", line);
        if line == "noop" {
            new_cycle(&mut cycle);
            check_frequency(&cycle, &register, &mut signal_strength);
        }
        else if let Ok(r) = line[5..].to_string().parse::<i32>() {
            //println!("{:?}", r);
            new_cycle(&mut cycle);
            check_frequency(&cycle, &register, &mut signal_strength);
            new_cycle(&mut cycle);
            register += r;
            check_frequency(&cycle, &register, &mut signal_strength);
        }
        else {
            panic!("cannot parse");
        }
    }

    println!("solution to problem one is {}", signal_strength);
    assert_eq!(signal_strength, 12520);
}
