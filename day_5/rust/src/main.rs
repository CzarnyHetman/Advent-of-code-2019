use std::env;
use std::io::{self, *};
use std::fs;
use std::path;
use int_comp::process_stream;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");

    let mut program = get_file().unwrap_or(vec![99]);

    process_stream(&mut program, &mut io::stdin().lock(), &mut io::stdout());
}

fn get_file() -> Option<Vec<i32>> {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

    let path = &args.get(1).expect("Supply path param");
    println!("{}", path);

    let path = path::Path::new(path);
    if !path.exists() {
        println!("Path unreachable");
        return None;
    }

    let file = fs::read_to_string(path).ok()?;

    let code : Vec<i32> = file.split(",").map(|line| line.trim().parse::<i32>().unwrap_or_default()).collect();
    
    Some(code)
}