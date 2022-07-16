pub struct Opcode {
    pub param_count: u8,
    pub param_config: Vec<ParamMode>
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum ParamMode {
    Position,
    Immediate
}

impl ParamMode {
    fn default() -> Self{
        ParamMode::Position
    }
}

#[derive(Debug)]
pub struct Param {
    pub index: usize,
    pub value: i32, 
    pub config: ParamMode
}

impl Param {
    pub fn get_value(&self, program: &Vec<i32>) -> i32 {
        let value = *match self.config {
            ParamMode::Position => program.get(self.value as usize).expect("Index out of bounds"),
            ParamMode::Immediate => &self.value
        };

        value
    }
    pub fn set_value(&self, program: &mut Vec<i32>, value: i32) {
        match self.config {
            ParamMode::Position => program[self.value as usize] = value,
            ParamMode::Immediate => program[self.index] = value
        }
    }

    pub fn get_params(program: &Vec<i32>, index: &usize, oc: &Opcode) -> Vec<Param> {
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