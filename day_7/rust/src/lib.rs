use std::{io::{BufRead, Write}, env, path, fs};

use instruction::{Status, Instruction, param::{Param, Opcode}};

pub mod instruction;

pub struct IntComp {
    const_program: Vec<i32>,
    program: Vec<i32>,
    index: usize,
    pub status: Status,
    oc: Option<Opcode>,
}

impl IntComp {
    pub fn new(program: &Vec<i32>) -> Self {
        IntComp { const_program: program.clone(), program: program.clone(), index: 0, status: Status::Ready, oc: None }
    }

    pub fn get_program(&self) -> Vec<i32> {
        self.program.clone()
    }

    pub fn reset(&mut self) -> Status {
        let original_program = self.const_program.clone();
        self.program = original_program;
        self.index = 0;
        self.oc = None;
        self.status = Status::Ready;

        Status::Ready
    }

    pub fn run(&mut self) -> Status {
        if self.status == Status::Halted || self.status == Status::RequestedInput {
            return self.status;
        }

        'run_loop: loop {
            let status = self.process_instruction();

            match status {
                Status::Halted => break 'run_loop Status::Halted,
                Status::Ready => continue,
                Status::Outputed(value) => break 'run_loop Status::Outputed(value),
                Status::RequestedInput => break 'run_loop Status::RequestedInput
            }
        }
    }

    pub fn run_with_input(&mut self, input: i32) -> Status{
        if self.status != Status::RequestedInput {
            return self.status;
        }

        let mut index = self.index;
        let mut program = &mut self.program;
        let oc = &self.oc.as_ref().unwrap();

        let params = Param::get_params(program, &index, &oc);
    
        params[0].set_value(&mut program, input);

        index += oc.param_count as usize;
        self.status = Status::Ready;
        self.index = index;
        self.run()
    }

    fn process_instruction(&mut self) -> Status {
        let mut index = self.index;
        let mut program = &mut self.program;
        let mut opcode = None;
        let inst = Instruction::new(program.get(index).expect("instruction missing"));
        index += 1;

        let status = match inst {
            Instruction::Add(oc) => {
                let params = Param::get_params(&program, &index, &oc);
    
                let val1 = params[0].get_value(&program);
                let val2 = params[1].get_value(&program);
                params[2].set_value(&mut program, val1 + val2);
    
                index += oc.param_count as usize;
                Status::Ready
            },
            Instruction::Multiply(oc) => {
                let params = Param::get_params(&program, &index, &oc);
    
                let val1 = params[0].get_value(&program);
                let val2 = params[1].get_value(&program);
                params[2].set_value(&mut program, val1 * val2);
    
                index += oc.param_count as usize;
                Status::Ready
            },
            Instruction::Input(oc) => {
                opcode = Some(oc);
                Status::RequestedInput
            },
            Instruction::Output(oc) => {
                let params = Param::get_params(&program, &index, &oc);
    
                let value = params[0].get_value(&program);
                
                index += oc.param_count as usize;
                Status::Outputed(value)
            },
            Instruction::JumpTrue(oc) => {
                let params = Param::get_params(&program, &index, &oc);
    
                let val1 = params[0].get_value(&program);
                let val2 = params[1].get_value(&program);
    
                if val1 != 0 {
                    index = val2 as usize;
                } else {
                    index += oc.param_count as usize;
                }
                Status::Ready
            },
            Instruction::JumpFalse(oc) => {
                let params = Param::get_params(&program, &index, &oc);
    
                let val1 = params[0].get_value(&program);
                let val2 = params[1].get_value(&program);
    
                if val1 == 0 {
                    index = val2 as usize;
                } else {
                    index += oc.param_count as usize;
                }
                Status::Ready
            },
            Instruction::LessThan(oc) => {
                let params = Param::get_params(&program, &index, &oc);
    
                let val1 = params[0].get_value(&program);
                let val2 = params[1].get_value(&program);
    
                if val1 < val2 {
                    params[2].set_value(&mut program, 1);
                } else {
                    params[2].set_value(&mut program, 0);
                }
    
                index += oc.param_count as usize;
                Status::Ready
            },
            Instruction::Equals(oc) => {
                let params = Param::get_params(&program, &index, &oc);
    
                let val1 = params[0].get_value(&program);
                let val2 = params[1].get_value(&program);
    
                if val1 == val2 {
                    params[2].set_value(&mut program, 1);
                } else {
                    params[2].set_value(&mut program, 0);
                }
                
                index += oc.param_count as usize;
                Status::Ready
            },
            Instruction::Halt => {
                Status::Halted
            }
        };
        self.oc = opcode;
        self.status = status;
        self.index = index;
        
        status
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

    mod int_comp_tests {
        use super::*;

        #[test]
        fn int_comp_adds() {
            let program = vec![10001, 1,1,0, 99];
            let mut int_comp = IntComp::new(&program);

            int_comp.run();

            assert_eq!(int_comp.get_program(), vec![10001, 1,1,2, 99])
        }

        #[test]
        fn int_comp_multiplies() {
            let program = vec![10002, 1,1,0, 99];
            let mut int_comp = IntComp::new(&program);

            int_comp.run();

            assert_eq!(int_comp.get_program(), vec![10002, 1,1,1, 99])
        }

        #[test]
        fn int_comp_resets() {
            let program = vec![10001, 1,1,0, 99];
            let mut int_comp = IntComp::new(&program);

            let status = int_comp.run();

            assert_eq!(status, Status::Halted);
            assert_eq!(int_comp.get_program(), vec![10001, 1,1,2, 99]);

            let status = int_comp.reset();

            assert_eq!(status, Status::Ready);
            assert_eq!(int_comp.get_program(), program);
        }

        #[test]
        fn int_comp_outputs() {
            let program = vec![104, 5, 99];
            let mut int_comp = IntComp::new(&program);

            let status = int_comp.run();

            assert_eq!(int_comp.get_program(), vec![104, 5, 99]);
            assert_eq!(status, Status::Outputed(5));

            let status = int_comp.run();

            assert_eq!(status, Status::Halted);
        }

        #[test]
        fn int_comp_inputs() {
            let program = vec![103, 5, 99];
            let mut int_comp = IntComp::new(&program);

            let status = int_comp.run();

            assert_eq!(status, Status::RequestedInput);

            let status = int_comp.run();

            assert_eq!(status, Status::RequestedInput);

            let status = int_comp.run_with_input(13);

            assert_eq!(status, Status::Halted);
            assert_eq!(int_comp.get_program(), vec![103, 13, 99]);
        }
    }


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
}