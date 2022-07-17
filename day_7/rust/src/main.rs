use int_comp::IntComp;
use int_comp;
use int_comp::instruction::Status;

fn main() {
    let program: Vec<i32> = int_comp::get_file().unwrap();

    let result = get_thruster_value_v1(&program);

    println!("Biggest output: {}, biggest phases: {:?}", result.0, result.1);

    let result = get_thruster_value_v2(&program);

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
        output = test_phases(program, &phases);
        if output > biggest_output {
            biggest_output = output;
            biggest_phases = phases.clone();
        }
    }

    (biggest_output, biggest_phases)
}

fn test_phases(program: &Vec<i32>, phases: &Vec<i32>) -> i32 {
    let mut comp: Vec<IntComp> = vec![
        IntComp::new(program),
        IntComp::new(program),
        IntComp::new(program),
        IntComp::new(program),
        IntComp::new(program)
    ];

    for (i, phase) in phases.iter().enumerate() {
        comp[i].run();
        let status = comp[i].run_with_input(*phase);
    }

    let mut input = 0;

    'main: loop {
        for c in comp.iter_mut() {
            let status = c.run_with_input(input);
            match status {
                Status::Outputed(output) => { input = output; c.run(); continue; },
                _ => break 'main
            }
            //println!("Input {}", input);
        }
    }
    input
}

fn get_thruster_value_v1(program: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut aggregate: Vec<Vec<i32>> = Vec::new();
    let mut permutable = vec![0, 1, 2, 3, 4];
    let k = permutable.len();
    
    let mut comp: Vec<IntComp> = vec![
        IntComp::new(program),
        IntComp::new(program),
        IntComp::new(program),
        IntComp::new(program),
        IntComp::new(program)
    ];
    generate_permutation(&mut aggregate, k, &mut permutable);

    let mut biggest_output = 0;
    let mut biggest_phases = permutable;

    for phases in aggregate {
        let mut input = 0;
        for (i, phase) in phases.iter().enumerate() {
            comp[i].reset();
            comp[i].run();
            comp[i].run_with_input(*phase);
            let status = comp[i].run_with_input(input);
            
            match status {
                Status::Outputed(output) => input = output,
                _ => continue
            }
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


#[cfg(test)]
mod test {
    use super::*;

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
