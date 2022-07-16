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

    let code : Vec<i32> = file.split(",").map(|line| line.trim().parse::<i32>().unwrap_or_default()).collect();
    
    //38, 92
    for p1 in (38..39){
        for p2 in (80..120){
            let mut codeCopy = code.clone();
            supplyParams(&mut codeCopy, p1, p2);
            process(&mut codeCopy);
            println!("For {}, {}, value is {}", p1, p2, &codeCopy[0]);
        }
    }

    Ok(())
}

fn supplyParams(code: &mut Vec<i32>, p1 : i32, p2 : i32){
    code[1] = p1;
    code[2] = p2;
}

fn process(code : &mut Vec<i32>){
    let mut index = 0;
    loop {
        let instruction = code.get(index);

        if instruction.is_none() {
            break;
        }
        let instruction = instruction.unwrap();

        match instruction {
            1 => {
                if index + 3 >= code.len() {
                    break;
                }
                let first_position: usize = *code.get(index + 1).expect("first position not found") as usize;
                let second_position: usize = *code.get(index + 2).expect("second position not found") as usize;
                let third_position: usize = *code.get(index + 3).expect("third position not found") as usize;

                let first = code.get(first_position).unwrap();
                let second = code[second_position];
                code[third_position] = first + second;

                index += 4;
            },
            2 => {
                if index + 3 >= code.len() {
                    break;
                }
                let first_position: usize = *code.get(index + 1).expect("first position not found") as usize;
                let second_position: usize = *code.get(index + 2).expect("second position not found") as usize;
                let third_position: usize = *code.get(index + 3).expect("third position not found") as usize;

                let first = code.get(first_position).unwrap();
                let second = code[second_position];
                code[third_position] = first * second;

                index += 4;
            },
            99 => { 
                break;
            },
            number => panic!("Instruction {} not supported", number)
        };
        
        if index >= code.len() {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_should_halt_on_99() {
        let mut code = vec![99,0,0,0,0,0,0];
        process(&mut code);

        assert_eq!(code, vec![99,0,0,0,0,0,0]);
    }

    #[test]
    #[should_panic(expected = "Instruction 0 not supported")]
    fn process_should_panic_on_incorrect_instruction() {
        let mut code = vec![0,0,0,0,0,0,0];
        process(&mut code);
    }

    #[test]
    fn process_should_add_on_one() {
        let mut code = vec![1,5,6,6,99,10,20];
        process(&mut code);

        assert_eq!(code, vec![1,5,6,6,99,10,30]);
    }

    #[test]
    fn process_should_multiply_on_two() {
        let mut code = vec![2,5,6,6,99,10,20];
        process(&mut code);

        assert_eq!(code, vec![2,5,6,6,99,10,200]);
    }
}
