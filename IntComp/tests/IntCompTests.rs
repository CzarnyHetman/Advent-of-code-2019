use IntComp::IntComp;
use ::IntComp::instruction::Status;

#[test]
fn self_replicating_program() {
    let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    let mut output = Vec::new();
    let mut int_comp = IntComp::new(&program);
    let status: Status = loop {
        let status = int_comp.run();

        match status {
            Status::Outputed(value) => output.push(value),
            _ => break status
        };
    };

    assert_eq!(output, program);
}
