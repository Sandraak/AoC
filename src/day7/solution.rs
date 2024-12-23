pub fn solve() {
    const INPUT: &str = include_str!("./puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");
    let calibration_equations = input_to_calibration_equition(INPUT);
    let sum = get_sum_of_correct_calibrations(calibration_equations);
    print!("Sum of correct calibrations: {}", sum);
}

#[derive(Debug)]
struct CalibrationEquation {
    result: u64,
    vector: Vec<u64>,
}

fn input_to_calibration_equition(input: &str) -> Vec<CalibrationEquation> {
    let mut result: u64 = 0;
    let mut vector: Vec<u64> = Vec::new();
    let mut calibration_equations: Vec<CalibrationEquation> = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_once(": ").unwrap();
        result = a.parse().unwrap();
        vector = b.split(" ").map(|x| x.parse().unwrap()).collect();
        calibration_equations.push(CalibrationEquation { result, vector });
    }
    calibration_equations
}

fn concatenate(current: u64, next: u64) -> u64 {
    format!("{}{}", current, next).parse().unwrap()
}

fn check_calibration(index: usize, current_value: u64, equation: &CalibrationEquation) -> bool {
    if index == equation.vector.len() {
        current_value == equation.result
    } else {
        check_calibration(index + 1, current_value + equation.vector[index], equation)
            || check_calibration(index + 1, current_value * equation.vector[index], equation)
            || check_calibration(
                index + 1,
                concatenate(current_value, equation.vector[index]),
                equation,
            )
    }
}

fn get_sum_of_correct_calibrations(calibration_equations: Vec<CalibrationEquation>) -> u64 {
    calibration_equations
        .into_iter()
        .filter(|equation| check_calibration(1, equation.vector[0], equation))
        .map(|equation| equation.result)
        .sum()
}
