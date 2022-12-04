use std::fs;
use std::cmp::min;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    problem1();
    problem2();
    problem2_with_binary_heap();
}

fn problem2_with_binary_heap(){
    let file_path = "src/input1_1.txt";
    let content = fs::read_to_string(file_path).expect("should be able to read the file");
    let lines =  content.split('\n');
    const MAX_SUMS_NUMBER :usize = 3;
    let mut max_sums = BinaryHeap::new();
    let mut current_sum = 0;
    for mut line in lines {
        line = line.trim_end();
        if line == ""
        {
            max_sums.push(Reverse(current_sum));
            if max_sums.len() > MAX_SUMS_NUMBER{
                max_sums.pop();
            }
            current_sum = 0;
        }
        else
        {
            let calories: i32 = line.parse().unwrap();
            current_sum += calories;
        }
    }
    println!("Problem 2 solved with binary heap : {}", max_sums.drain().map(|sum| sum.0).sum::<i32>());
}

fn problem2(){
    let file_path = "src/input1_1.txt";
    let content = fs::read_to_string(file_path).expect("should be able to read the file");
    let lines =  content.split('\n');
    const MAX_SUMS_NUMBER :usize = 3;
    let mut current_sum = 0;
    let mut max_sums = Vec::new();
    for mut line in lines {
        line = line.trim_end();
        if line == ""
        {
            if max_sums.len() == 0 {
                max_sums.push(current_sum)
            }
            else{
                for i in (0..min(MAX_SUMS_NUMBER, max_sums.len())).rev() {
                    if current_sum > max_sums[i] && (i == 0 || current_sum <= max_sums[i-1]) {
                        max_sums.insert(i, current_sum);
                        if max_sums.len() > MAX_SUMS_NUMBER {
                            max_sums.pop();
                        }
                        break;
                    }
                }
            }
            current_sum = 0;
        }
        else
        {
            let calories: i32 = line.parse().unwrap();
            current_sum += calories;
        }
    }
    println!("Problem 2 : {}", max_sums.iter().sum::<i32>());
}

fn problem1(){
    let file_path = "src/input1_1.txt";
    let content = fs::read_to_string(file_path).expect("should be able to read the file");
    let lines =  content.split('\n');
    let mut current_sum = 0;
    let mut max_sum = 0;
    for mut line in lines
    {
        line = line.trim_end();
        if line == ""
        {
            if current_sum > max_sum
            {
                max_sum = current_sum;
            }
            current_sum = 0;
        }
        else
        {
            let calories: i32 = line.parse().unwrap();
            current_sum += calories;
        }
    }
    println!("Problem 1 : {}",max_sum.to_string());
}

//complicado
// fn read_input() -> std::str::Split<char>
// {
//     let file_path = "src/input1_1.txt";
//     let content = fs::read_to_string(file_path).expect("should be able to read the file");
//     return content.split('\n');
// }