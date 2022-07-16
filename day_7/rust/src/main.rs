use int_comp;

struct Amplifier {
    program: Vec<i32>,
    phase: i32
}

impl Amplifier {
    fn new(program: &Vec<i32>, phase: i32) -> Self {
        Amplifier { program: program.clone(), phase }
    }

    fn run<F> (&mut self, mut input: F) -> i32
    where F: FnMut() -> i32 
    {
        let phase = self.phase;
        let mut phase_supplied = false;
        let mut output = String::new();

        int_comp::process(&mut self.program, || {
            if !phase_supplied {
                phase_supplied = true;
                return phase.to_string();
            }
            input().to_string()
        }, |s| output = s);

        output.parse().unwrap()
    }
}

fn main() {
    let program: Vec<i32> = int_comp::get_file().unwrap();

    let result = get_thruster_value_v1(&program);

    println!("Biggest output: {}, biggest phases: {:?}", result.0, result.1);
}

fn get_thruster_value_v2(program: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut aggregate: Vec<Vec<i32>> = Vec::new();
    let mut permutable = vec![5, 6, 7, 8, 9];
    let k = permutable.len();
    generate_permutation(&mut aggregate, k, &mut permutable);

    let mut biggest_output = 0;
    let mut biggest_phases = permutable;

    for phases in aggregate {
        let mut output = 0;
        output = test_phases(&phases);
        
        if output > biggest_output {
            biggest_output = output;
            biggest_phases = phases.clone();
        }
    }

    (biggest_output, biggest_phases)
}

fn test_phases(phases: &Vec<i32>) -> i32 {
    let mut amps: Vec<Amplifier> = Vec::new();
    

    
    2
}

fn get_thruster_value_v1(program: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut aggregate: Vec<Vec<i32>> = Vec::new();
    let mut permutable = vec![0, 1, 2, 3, 4];
    let k = permutable.len();
    generate_permutation(&mut aggregate, k, &mut permutable);

    let mut biggest_output = 0;
    let mut biggest_phases = permutable;

    for phases in aggregate {
        let mut input = 0;
        for phase in &phases {
            input = run_process(&program, *phase, input).parse().unwrap();
        }
        if input > biggest_output {
            biggest_output = input;
            biggest_phases = phases.clone();
        }
    }

    (biggest_output, biggest_phases)
}

fn generate_permutation(aggregate : &mut Vec<Vec<i32>>, k : usize, permutable: &mut Vec<i32>){
    if k == 1 {
        aggregate.push(permutable.clone());
        return;
    } else {
        generate_permutation(aggregate, k - 1, permutable);

        for i in 0..k-1 {
            if k % 2 == 0 {
                permutable.swap(i, k - 1);
            } else {
                permutable.swap(0, k - 1);
            }
            generate_permutation(aggregate, k - 1, permutable);
        }
    }
}

fn run_process(program: &Vec<i32>, phase: i32, input: i32) -> String{
    let mut program: Vec<i32> = program.clone();
    let mut phase_suplied = false;
    let mut output = String::new();

    int_comp::process(&mut program, || {
        if !phase_suplied {
            phase_suplied = true;
            return phase.to_string();
        }
        input.to_string()
    }, |s| output = s);

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_process_returns_0() {
        let program = vec![103,1,103,2,10001, 1, 3, 15, 4, 7, 99];

        let phase = 1;
        let input = -1;
        let result = run_process(&program, phase, input);

        assert_eq!(result, "0");
    }

    #[test]
    fn generate_permutation_outputs_permutations() {
        let mut aggregate: Vec<Vec<i32>> = Vec::new();
        let mut permutable = vec![1, 2, 3];
        let k = permutable.len();
        generate_permutation(&mut aggregate, k, &mut permutable);

        assert_eq!(aggregate, vec![vec![1, 2, 3], vec![2, 1, 3], vec![3, 1, 2], vec![1, 3, 2], vec![2, 3, 1], vec![3, 2, 1]]);
    }

    #[test]
    fn get_thruster_value_test_1(){
        let program: Vec<i32> = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let expected_output = 43210;
        let expected_phases = vec![4,3,2,1,0];

        let result = get_thruster_value_v1(&program);

        assert_eq!(result.0, expected_output);
        assert_eq!(result.1, expected_phases);
    }

    #[test]
    fn get_thruster_value_test_2(){
        let program: Vec<i32> = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let expected_output = 54321;
        let expected_phases = vec![0,1,2,3,4];

        let result = get_thruster_value_v1(&program);

        assert_eq!(result.0, expected_output);
        assert_eq!(result.1, expected_phases);
    }

    #[test]
    fn get_thruster_value_test_3(){
        let program: Vec<i32> = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let expected_output = 65210;
        let expected_phases = vec![1,0,4,3,2];

        let result = get_thruster_value_v1(&program);

        assert_eq!(result.0, expected_output);
        assert_eq!(result.1, expected_phases);
    }
}
