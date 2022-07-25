use IntComp::{self, instruction::Status};
use std::io::{ self, Read, BufRead };

fn main() {
    let program = IntComp::get_program_from_file().unwrap();

    let mut int_comp = IntComp::IntComp::new(&program);

    loop {
        let status = int_comp.run();

        match status {
            Status::Outputed(number) => println!("Output: {}", number),
            Status::RequestedInput => {
                println!("Input requested");
                let mut buffer = String::new();
                let stdin = io::stdin();
                let mut handle = stdin.lock();

                handle.read_line(&mut buffer).unwrap();
                let value: i64 = buffer.trim().parse().unwrap();
                println!("Supplied: {}", value);
                int_comp.run_with_input(value);
            },
            _ => break
        }
    }
}
