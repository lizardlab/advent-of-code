use std::{env, fs};
use fancy_regex::Regex;
fn main() {
    let args: Vec<String>  = env::args().collect();
    if args.len() != 2{
        eprintln!("Provide ranges");
    }
    let filename = args.get(1).unwrap();
    let barcodes = fs::read_to_string(filename).expect("File error");
    let ranges: Vec<_> = barcodes.trim().split(',').collect();
    let mut part1:i64 = 0;
    let mut part2:i64 = 0;
    let repeat_match = Regex::new(r"^(.+)(?:\1)+$").unwrap();
    for rng in ranges.iter() {
        let ends: Vec<_> = rng.split('-').collect();
        if ends[1] != ""{
            let start = ends[0].parse::<i64>().unwrap();
            let end = ends[1].parse::<i64>().unwrap();
            //let diff = end - start;
            for i in start..end+1{
                let id = i.to_string();
                if id.len() % 2 == 0{
                    let first = id.get(0..id.len()/2).unwrap();
                    let last = id.get(id.len()/2..).unwrap();
                    if first == last{
                        part1 += i;
                    }
                }
                if repeat_match.is_match(&id).unwrap(){
                    part2 += i
                }
            }
        }
    }
    println!("Part 1: {part1:?}");
    println!("Part 2: {part2:?}");
}
