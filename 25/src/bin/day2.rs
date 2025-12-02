use std::{env, fs};
fn main() {
    let args: Vec<String>  = env::args().collect();
    if args.len() != 2{
        eprintln!("Provide ranges");
    }
    let filename = args.get(1).unwrap();
    let barcodes = fs::read_to_string(filename).expect("File error");
    let ranges: Vec<_> = barcodes.trim().split(',').collect();
    let mut count:i64 = 0;
    for rng in ranges.iter() {
        let ends: Vec<_> = rng.split('-').collect();
        if ends[1] != ""{
            let start = ends[0].parse::<i64>().unwrap();
            let end = ends[1].parse::<i64>().expect("asdf");
            //let diff = end - start;
            for i in start..end+1{
                let id = i.to_string();
                if id.len() % 2 == 0{
                    let first = id.get(0..id.len()/2).unwrap();
                    let last = id.get(id.len()/2..).unwrap();
                    if first == last{
                        count += i;
                    }
                }
            }
        }
    }
    println!("Part 1: {count:?}");
}
