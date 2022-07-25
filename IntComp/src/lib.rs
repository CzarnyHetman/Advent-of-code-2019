use std::{env, path, fs};

use instruction::{Status, Instruction, param::{Param, Opcode}};

pub mod instruction;

#[derive(Debug)]
pub struct Program {
    pub memory: Vec<i64>,
    index: usize,
    oc: Option<Opcode>,
    relative_base: usize,
    pub status: Status,
}

impl Program {
    fn new(program: Vec<i64>) -> Self {
        Program { memory: program, index: 0, oc: None, relative_base: 0, status: Status::Ready }
    }

    fn extend_memory_to(&mut self, size: usize){
        let length = self.memory.len();
        let mut extension = vec![0i64;size - length + 1];
        self.memory.append(&mut extension);
    }

    fn get(&mut self, index: usize) -> i64 {
        if self.memory.len() <= index {
            self.extend_memory_to(index);
        }

        *self.memory.get(index).unwrap()
    }

    fn set(&mut self, index: usize, value: i64){
        if self.memory.len() <= index {
            self.extend_memory_to(index);
        }

        self.memory[index] = value;
    }
}

pub struct IntComp {
    const_program: Vec<i64>,
    program: Program,
}

impl IntComp {
    pub fn new(program: &Vec<i64>) -> Self {
        IntComp { const_program: program.clone(), program: Program::new(program.clone()) }
    }

    pub fn get_program(&self) -> Vec<i64> {
        self.program.memory.clone()
    }

    pub fn reset(&mut self) -> Status {
        let original_program = self.const_program.clone();
        self.program =  Program::new(original_program);

        Status::Ready
    }

    pub fn run(&mut self) -> Status {
        if self.program.status == Status::Halted || self.program.status == Status::RequestedInput {
            return self.program.status;
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

    pub fn run_with_input(&mut self, input: i64) -> Status{
        if self.program.status != Status::RequestedInput {
            return self.program.status;
        }

        let oc = &self.program.oc.clone().unwrap();
        let index = self.program.index;
        let mut program = &mut self.program;

        let params = Param::get_params(program, &index, &oc);
    
        params[0].set_value(&mut program, input);
        println!("Index before: {}", index);
        let index = index + (oc.param_count as usize);
        println!("Index before: {}", index);
        self.program.status = Status::Ready;
        self.program.index = index;
        self.run()
    }

    fn process_instruction(&mut self) -> Status {
        let mut index = self.program.index;
        let mut opcode = None;
        let inst = Instruction::new(self.program.memory.get(index).expect("instruction missing"));
        index += 1;

        let status = match inst {
            Instruction::Add(oc) => {
                let params = Param::get_params(&self.program, &index, &oc);
    
                let val1 = params[0].get_value(&mut self.program);
                let val2 = params[1].get_value(&mut self.program);
                params[2].set_value(&mut self.program, val1 + val2);
    
                index += oc.param_count as usize;
                Status::Ready
            },
            Instruction::Multiply(oc) => {
                let params = Param::get_params(&self.program, &index, &oc);
    
                let val1 = params[0].get_value(&mut self.program);
                let val2 = params[1].get_value(&mut self.program);
                params[2].set_value(&mut self.program, val1 * val2);
    
                index += oc.param_count as usize;
                Status::Ready
            },
            Instruction::Input(oc) => {
                opcode = Some(oc);
                Status::RequestedInput
            },
            Instruction::Output(oc) => {
                let params = Param::get_params(&self.program, &index, &oc);
    
                let value = params[0].get_value(&mut self.program);
                
                index += oc.param_count as usize;
                Status::Outputed(value)
            },
            Instruction::JumpTrue(oc) => {
                let params = Param::get_params(&self.program, &index, &oc);
    
                let val1 = params[0].get_value(&mut self.program);
                let val2 = params[1].get_value(&mut self.program);
    
                if val1 != 0 {
                    index = val2 as usize;
                } else {
                    index += oc.param_count as usize;
                }
                Status::Ready
            },
            Instruction::JumpFalse(oc) => {
                let params = Param::get_params(&self.program, &index, &oc);
    
                let val1 = params[0].get_value(&mut self.program);
                let val2 = params[1].get_value(&mut self.program);
    
                if val1 == 0 {
                    index = val2 as usize;
                } else {
                    index += oc.param_count as usize;
                }
                Status::Ready
            },
            Instruction::LessThan(oc) => {
                let params = Param::get_params(&self.program, &index, &oc);
    
                let val1 = params[0].get_value(&mut self.program);
                let val2 = params[1].get_value(&mut self.program);
    
                if val1 < val2 {
                    params[2].set_value(&mut self.program, 1);
                } else {
                    params[2].set_value(&mut self.program, 0);
                }
    
                index += oc.param_count as usize;
                Status::Ready
            },
            Instruction::Equals(oc) => {
                let params = Param::get_params(&self.program, &index, &oc);
    
                let val1 = params[0].get_value(&mut self.program);
                let val2 = params[1].get_value(&mut self.program);
    
                if val1 == val2 {
                    params[2].set_value(&mut self.program, 1);
                } else {
                    params[2].set_value(&mut self.program, 0);
                }
                
                index += oc.param_count as usize;
                Status::Ready
            },
            Instruction::AdjustRelativeBase(oc) => {
                let params = Param::get_params(&self.program, &index, &oc);

                let val1 = params[0].get_value(&mut self.program);

                self.program.relative_base += val1 as usize;

                index += oc.param_count as usize;
                Status::Ready
            }
            Instruction::Halt => {
                Status::Halted
            }
        };
        self.program.oc = opcode;
        self.program.status = status;
        self.program.index = index;
        
        status
    }
    
}

pub fn get_program_from_file() -> Option<Vec<i64>> {
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

    let code : Vec<i64> = file.split(",").map(|line| line.trim().parse::<i64>().unwrap_or_default()).collect();
    
    Some(code)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extend_memory_to_5_extends_to_len_6() {
        let mut program = Program::new(vec![1]);
        program.extend_memory_to(5);

        assert_eq!(program.memory.len(), 6);
    }

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