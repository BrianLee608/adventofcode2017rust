use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u32 = args[1].parse::<u32>().expect("provide an integer");
    
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        _ => println!("Not implemented yet"),
    }
}
