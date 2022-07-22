use self::param::{ParamMode, Opcode};

pub mod param;

#[derive(PartialEq, Clone, Copy)]
#[derive(Debug)]
pub enum Status {
    Ready,
    RequestedInput,
    Outputed(i64),
    Halted
}

#[derive(Debug)]
pub enum Instruction {
    Add(Opcode),
    Multiply(Opcode),
    Input(Opcode),
    Output(Opcode),
    JumpTrue(Opcode),
    JumpFalse(Opcode),
    LessThan(Opcode),
    Equals(Opcode),
    AdjustRelativeBase(Opcode),
    Halt
}

impl Instruction {
    pub fn new(opcode: &i64) -> Self{
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
            9 => Instruction::AdjustRelativeBase(Opcode {param_count: 1, param_config: opcode.1}),
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
                    2 => ParamMode::Relative,
                    number => panic!("{} is invalid parameter mode", number)
                });
            }
        }

        (oc, pc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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