use crate::Program;

#[derive(Clone)]
pub struct Opcode {
    pub param_count: u8,
    pub param_config: Vec<ParamMode>
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum ParamMode {
    Position,
    Immediate,
    Relative
}

impl ParamMode {
    fn default() -> Self{
        ParamMode::Position
    }
}

#[derive(Debug)]
pub struct Param {
    pub index: usize,
    pub value: i64, 
    pub config: ParamMode
}

impl Param {
    pub fn get_value(&self, program: &Program) -> i64 {
        let value = *match self.config {
            ParamMode::Position => program.memory.get(self.value as usize).expect("Index out of bounds"),
            ParamMode::Relative => program.memory.get(program.relative_base + self.value as usize).expect("Index out of bounds"),
            ParamMode::Immediate => &self.value,
        };

        value
    }
    pub fn set_value(&self, program: &mut Program, value: i64) {
        let relative_base = program.relative_base;
        match self.config {
            ParamMode::Position => program.memory[self.value as usize] = value,
            ParamMode::Relative => program.memory[relative_base + self.value as usize] = value,
            ParamMode::Immediate => program.memory[self.index] = value
        }
    }

    pub fn get_params(program: &Program, index: &usize, oc: &Opcode) -> Vec<Param> {
        let mut params: Vec<Param> = Vec::new();
        for i in 0..oc.param_count {
            let i = i as usize;
            let value = *program.memory.get(index + i).expect("instruction missing");
            let config = oc.param_config.get(i).unwrap_or(&ParamMode::default()).clone();
            params.push(Param {index: index + i, value, config })
        };
        params
    }
}