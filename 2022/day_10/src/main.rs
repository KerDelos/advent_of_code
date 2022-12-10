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

fn print_screen(screen :&Vec<char>)
{
    for y in 0..6 {
        for x in 0..40{
            print!("{}",screen[x+y*40]);
        }
        println!("");
    }
}

fn draw_screen(screen :&mut Vec<char>,cycle: &i32, register: &i32){
    let x = (cycle-1) % 40;
    let y = (cycle-1) / 40; //TODO why are parantheses necessary here ??
    if x >= register - 1 && x <= register + 1 {
        let idx = usize::try_from(x+y*40).unwrap();
        if idx >= 240{
            print_screen(screen);
        }
        screen[idx] = '#';
    }
}

fn main() {
    let content = std::fs::read_to_string("src/input_1.txt").unwrap();

    let mut cycle = 1;
    let mut register = 1;
    let mut signal_strength = 0;
    let mut screen = vec!['.'; 40*6];
    for line in content.lines(){
        //println!("{:?}", line);
        if line == "noop" {
            draw_screen(&mut screen, &cycle, &register);
            new_cycle(&mut cycle);
            check_frequency(&cycle, &register, &mut signal_strength);
        }
        else if let Ok(r) = line[5..].to_string().parse::<i32>() {
            //println!("{:?}", r);
            draw_screen(&mut screen, &cycle, &register);
            new_cycle(&mut cycle);
            check_frequency(&cycle, &register, &mut signal_strength);

            draw_screen(&mut screen, &cycle, &register);
            new_cycle(&mut cycle);
            register += r;
            check_frequency(&cycle, &register, &mut signal_strength);
        }
        else {
            panic!("cannot parse");
        }
    }

    println!("solution to problem one is {}", signal_strength);
    //assert_eq!(signal_strength, 12520);
    print_screen(&screen);
}
