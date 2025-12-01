use std::{env, fs};
fn main() {
    let args: Vec<String>  = env::args().collect();
    if args.len() != 2{
        eprintln!("Provide rotations");
    }
    let filename = args.get(1).unwrap();
    let rotations = fs::read_to_string(filename).expect("File error");
    let mut rots: Vec<_> = rotations.split('\n').collect();
    rots = rots[..rots.len()-1].to_vec();
    let mut dial:i32 = 50;
    let mut count:i32 = 0;
    for rot in rots.iter() {
        let dir = rot.get(0..1).unwrap();
        let dist = rot.get(1..).unwrap();
        let mut dist_num = dist.parse::<i32>().unwrap();
        count += dist_num / 100;
        dist_num %= 100;
        if dir == "L"{
            dist_num *= -1;
        }
        dial += dist_num;
        //println!("Dial: {}\tDistNum: {}", dial, dist_num);
        if dial < 0{
            dial += 100;
            count += 1;
        }
        //count += dial / 100;
        dial %= 100;
        if dial == 0{
            count += 1;
        }
    }
    println!("{}", count);
}
