use std::{io::{BufRead, Write}, env, path, fs};

enum Instruction {
    Add(Opcode),
    Multiply(Opcode),
    Input(Opcode),
    Output(Opcode),
    JumpTrue(Opcode),
    JumpFalse(Opcode),
    LessThan(Opcode),
    Equals(Opcode),
    Halt
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum ParamMode {
    Position,
    Immediate
}

impl ParamMode {
    fn default() -> Self{
        ParamMode::Position
    }
}

#[derive(Debug)]
struct Param {
    index: usize,
    value: i32, 
    config: ParamMode
}

impl Param {
    fn get_value(&self, program: &Vec<i32>) -> i32 {
        let value = *match self.config {
            ParamMode::Position => program.get(self.value as usize).expect("Index out of bounds"),
            ParamMode::Immediate => &self.value
        };

        value
    }
    fn set_value(&self, program: &mut Vec<i32>, value: i32) {
        match self.config {
            ParamMode::Position => program[self.value as usize] = value,
            ParamMode::Immediate => program[self.index] = value
        }
    }

    fn get_params(program: &Vec<i32>, index: &usize, oc: &Opcode) -> Vec<Param> {
        let mut params: Vec<Param> = Vec::new();
        for i in 0..oc.param_count {
            let i = i as usize;
            let value = *program.get(index + i).expect("instruction missing");
            let config = oc.param_config.get(i).unwrap_or(&ParamMode::default()).clone();
            params.push(Param {index: index + i, value, config })
        };
        params
    }
}


struct Opcode {
    param_count: u8,
    param_config: Vec<ParamMode>
}

impl Instruction {
    pub fn new(opcode: &i32) -> Self{
        let opcode = Instruction::parse_opcode(&opcode.to_string());

        match opcode.0 {
            1 => Instruction::Add(Opcode {param_count: 3, param_config: opcode.1 }),
            2 => Instruction::Multiply(Opcode {param_count: 3, param_config: opcode.1 }),
            3 => Instruction::Input(Opcode {param_count: 1, param_config: opcode.1 }),
            4 => Instruction::Output(Opcode {param_count: 1, param_config: opcode.1 }),
            5 => Instruction::JumpTrue(Opcode {param_count: 2, param_config: opcode.1}),
            6 => Instruction::JumpFalse(Opcode {param_count: 2, param_config: opcode.1}),
            7 => Instruction::LessThan(Opcode {param_count: 3, param_config: opcode.1}),
            8 => Instruction::Equals(Opcode {param_count: 3, param_config: opcode.1}),
            99 => Instruction::Halt,
            number => panic!("{} is not supported opcode", number),

        }
    }

    fn parse_opcode(opcode: &str) -> (u8, Vec<ParamMode>){
        let len = opcode.len();
        let mut oc = 0;
        let mut pc = Vec::new();
        if len < 2 {
            oc = opcode.parse().expect("Opcode must be int");
        } else {
            oc = opcode[len-2..].parse().expect("Opcode must be int");
            for char in opcode[..len-2].chars().rev() {
                let i: u8 = char.to_digit(10).unwrap() as u8;
                pc.push(match i {
                    0 => ParamMode::Position,
                    1 => ParamMode::Immediate,
                    number => panic!("{} is invalid parameter mode", number)
                });
            }
        }

        (oc, pc)
    }
}

pub fn process_stream<T: BufRead , U: Write>(program: &mut Vec<i32>, input: &mut T, output: &mut U){
    process(program, || {
        let mut buf = String::new();
        input.read_line(&mut buf);
        let s = buf.trim();
        return String::from(s);
    }, |value| {
        let mut value = String::from(value);
        value.push_str("\n\r");
        let mut value = value.as_bytes();

        output.write_all(&mut value).expect("write failed");
    })

}

pub fn process<'a, F, G> (program: &mut Vec<i32>, mut input : F, mut output : G)
where F: FnMut() -> String,
    G: FnMut(String)
{
    let mut index = 0usize;

    'process_loop: loop {
        let inst = Instruction::new(program.get(index).expect("instruction missing"));
        index += 1;
        match inst {
            Instruction::Add(oc) => {
                let params = Param::get_params(program, &index, &oc);

                let val1 = params[0].get_value(program);
                let val2 = params[1].get_value(program);
                params[2].set_value(program, val1 + val2);

                index += oc.param_count as usize;
            },
            Instruction::Multiply(oc) => {
                let params = Param::get_params(program, &index, &oc);

                let val1 = params[0].get_value(program);
                let val2 = params[1].get_value(program);
                params[2].set_value(program, val1 * val2);

                index += oc.param_count as usize;
            },
            Instruction::Input(oc) => {
                let params = Param::get_params(program, &index, &oc);

                let s = input();
                let value = s.parse::<i32>().expect("Input was wrong");

                params[0].set_value(program, value);

                index += oc.param_count as usize;
            },
            Instruction::Output(oc) => {
                let params = Param::get_params(program, &index, &oc);

                let value = params[0].get_value(program).to_string();
                
                output(value);

                index += oc.param_count as usize;
            },
            Instruction::JumpTrue(oc) => {
                let params = Param::get_params(program, &index, &oc);

                let val1 = params[0].get_value(program);
                let val2 = params[1].get_value(program);

                if val1 != 0 {
                    index = val2 as usize;
                } else {
                    index += oc.param_count as usize;
                }
            },
            Instruction::JumpFalse(oc) => {
                let params = Param::get_params(program, &index, &oc);

                let val1 = params[0].get_value(program);
                let val2 = params[1].get_value(program);

                if val1 == 0 {
                    index = val2 as usize;
                } else {
                    index += oc.param_count as usize;
                }
            },
            Instruction::LessThan(oc) => {
                let params = Param::get_params(program, &index, &oc);

                let val1 = params[0].get_value(program);
                let val2 = params[1].get_value(program);

                if val1 < val2 {
                    params[2].set_value(program, 1);
                } else {
                    params[2].set_value(program, 0);
                }

                index += oc.param_count as usize;
            },
            Instruction::Equals(oc) => {
                let params = Param::get_params(program, &index, &oc);

                let val1 = params[0].get_value(program);
                let val2 = params[1].get_value(program);

                if val1 == val2 {
                    params[2].set_value(program, 1);
                } else {
                    params[2].set_value(program, 0);
                }
                
                index += oc.param_count as usize;
            },
            Instruction::Halt => {
                break 'process_loop;
            }
        }
    }
}

pub fn get_file() -> Option<Vec<i32>> {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_adds() {
        let mut program = vec![10001, 1,1,0, 99];
        let mut output_buf: Vec<u8> = Vec::new();

        process_stream(&mut program, &mut "".as_bytes(), &mut output_buf);

        assert_eq!(program, vec![10001, 1,1,2, 99])
    }

    #[test]
    fn process_multiplies() {
        let mut program = vec![10002, 1,1,0, 99];
        let mut output_buf: Vec<u8> = Vec::new();

        process_stream(&mut program, &mut "".as_bytes(), &mut output_buf);

        assert_eq!(program, vec![10002, 1,1,1, 99])
    }

    #[test]
    fn process_outputs() {
        let mut program = vec![104, 5, 99];
        let mut output_buf: Vec<u8> = Vec::new();

        process_stream(&mut program, &mut "".as_bytes(), &mut output_buf);

        assert_eq!(program, vec![104, 5, 99]);
        assert_eq!(output_buf, vec![53, 10, 13]);
    }

    #[test]
    fn process_inputs() {
        let mut program = vec![103, 5, 99];
        let mut output_buf: Vec<u8> = Vec::new();

        process_stream(&mut program, &mut "13".as_bytes(), &mut output_buf);

        assert_eq!(program, vec![103, 13, 99]);
    }

    #[test]
    fn process_inputs_positional() {
        let mut program = vec![3, 3, 99, 0];
        let mut output_buf: Vec<u8> = Vec::new();

        process_stream(&mut program, &mut "13".as_bytes(), &mut output_buf);

        assert_eq!(program, vec![3, 3, 99, 13]);
    }


    #[test]
    fn parse_opcode_parses_2() {
        let oc = Instruction::parse_opcode("2");

        assert_eq!(oc.0, 2);
    }

    #[test]
    fn parse_opcode_parses_02() {
        let oc = Instruction::parse_opcode("02");

        assert_eq!(oc.0, 2);
    }

    #[test]
    fn parse_opcode_parses_1199() {
        let oc = Instruction::parse_opcode("1199");

        assert_eq!(oc.0, 99);
        assert_eq!(oc.1, vec![ParamMode::Immediate; 2]);
    }

    #[test]
    fn parse_opcode_parses_10103() {
        let oc = Instruction::parse_opcode("10103");

        assert_eq!(oc.0, 3);
        assert_eq!(oc.1, vec![ParamMode::Immediate, ParamMode::Position, ParamMode::Immediate]);
    }
}

