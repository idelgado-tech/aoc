use std::{env, fs};

pub mod day1;
pub mod day2;


pub fn read_input (day: &str) -> String {

    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("resources").join(format!("input_{day}.txt"));
    print!("file path {:?}", filepath);
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")

}


pub fn read_exemple (day: &str) -> String {

    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("resources").join(format!("exemple_{day}.txt"));
    println!("file path {:?}", filepath);
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")

}