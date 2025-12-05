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
        let first_numbers: Vec<_> = first_char_space.split_terminator("").map(|x| if x != "" x.parse::<i32>() else { continue }).collect();
        let first_numbers = first_numbers.get(1..first_numbers.len());
        println!("{first_numbers:?}");
        let first_max = first_numbers.iter().max();
    }
    println!("Part 1: {part1:?}");
    println!("Part 2: {part2:?}");
}
