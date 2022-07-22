use crate::Program;

#[derive(Clone)]
#[derive(Debug)]
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
    pub fn get_value(&self, program: &mut Program) -> i64 {
        let position = self.value as usize;
        let relative = (program.relative_base as i64 + self.value) as usize;
        let value = match self.config {
            ParamMode::Position => program.get(position),
            ParamMode::Relative => program.get(relative),
            ParamMode::Immediate => self.value,
        };

        value
    }
    pub fn set_value(&self, program: &mut Program, value: i64) {
        let position = self.value as usize;
        let relative = (program.relative_base as i64 + self.value) as usize;
        match self.config {
            ParamMode::Position => program.set(position, value),
            ParamMode::Relative => program.set(relative, value),
            ParamMode::Immediate => program.set(self.index, value)
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