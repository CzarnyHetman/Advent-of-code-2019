use std::fs;
use std::env;
use std::path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

    let path = &args.get(1).expect("Supply path param");
    println!("{}", path);

    let path = path::Path::new(path);
    if !path.exists() {
        println!("Path unreachable");
        return Ok(())
    }


    let file = fs::read_to_string(path)?;
    let lines : Vec<&str> = file.split("\n").collect();
    //println!("number of lines: {}\n{:?}", lines.len(), lines);

    let fuel : i32 = lines.iter().map(|line| get_fuel_with_itself(&line.trim().parse::<i32>().unwrap_or_default())).sum();
    println!("{:?}",fuel);

    Ok(())
}

fn get_fuel(mass: &i32) -> i32 {
    if *mass == 0 {
        return 0
    }
    mass / 3 - 2
}

fn get_fuel_with_itself(mass: &i32) -> i32 {
    let mut fuel_mass = 0;
    let mut previous_mass = *mass;
    loop {
        let partial_fuel = get_fuel(&previous_mass);
        if partial_fuel <= 0 {
            break;
        }
        previous_mass = partial_fuel;
        fuel_mass += partial_fuel;
    }
    fuel_mass
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(12, 2)]
    #[case(14, 2)]
    #[case(1969, 654)]
    #[case(100756, 33583)]
    fn gets_fuel_for(#[case] input: i32, #[case] expected: i32) {
        let result = get_fuel(&input);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(14, 2)]
    #[case(1969, 966)]
    #[case(100756, 50346)]
    fn gets_fuel_with_itself_for(#[case] input: i32, #[case] expected: i32) {
        let result = get_fuel_with_itself(&input);
        assert_eq!(expected, result);

    }
}
