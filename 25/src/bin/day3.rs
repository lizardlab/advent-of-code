use std::{env, fs};
fn main() {
    let args: Vec<String>  = env::args().collect();
    if args.len() != 2{
        eprintln!("Provide batteries");
    }
    let filename = args.get(1).unwrap();
    let batteries = fs::read_to_string(filename).expect("File error");
    let joltages: Vec<_> = batteries.trim().split('\n').collect();
    let mut part1:i64 = 0;
    let mut part2:i64 = 0;
    for jolt in joltages.iter() {
        let first_char_space = jolt.get(..jolt.len()-1).unwrap();
        let numbers: Vec<_> = first_char_space.split_terminator("").filter_map(|x| x.parse::<i32>().ok()).collect();
        let first_numbers = numbers.get(1..numbers.len()).unwrap();
        let first_max = first_numbers.into_iter().max().unwrap();
        let cut = numbers.iter().position(|x| x == first_max).unwrap();
        let second_numbers = numbers.get(cut+1..).unwrap();
        let second_max = second_numbers.into_iter().max().unwrap();
        println!("Second Max: {second_max:?}");
    }
    println!("Part 1: {part1:?}");
    println!("Part 2: {part2:?}");
}
