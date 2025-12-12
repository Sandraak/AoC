use std::str::FromStr;

pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("test_input");
    let problems = parse_input(INPUT);
    let solution = solve_problems(&problems);
}

#[derive(Debug)]
struct Problem{
    numbers: Vec<usize>,
    operator: Operator,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn apply(self, a: Vec<usize>) -> usize {
        match self {
            Operator::Add => a.iter().sum(),
            Operator::Multiply => a.iter().product(),
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Multiply),
            _   => None,
        }
    }
}

impl FromStr for Operator {
    type Err = String; // or a custom error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _   => Err(format!("Invalid operator: {}", s)),
        }
    }
}

fn parse_input(input: &str) -> Vec<Problem> {
    let mut operations : Vec<Operator> = Vec::new();
    let mut all_numbers_in_row: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        if line.contains('+') || line.contains('*') {
            operations = line.split_whitespace().map(|operator| operator.parse::<Operator>().unwrap()).collect();
        }
        else {
            let numbers_in_row : Vec<usize>= line.split_whitespace().map(|number| number.parse::<usize>().unwrap()).collect();
            all_numbers_in_row.push(numbers_in_row);
        }
    }
    create_problems(&all_numbers_in_row, &operations)
}

fn create_problems(all_numbers_in_row: &Vec<Vec<usize>>, operators_in_row: &Vec<Operator>) -> Vec<Problem> {
    let mut problems : Vec<Problem> = Vec::new();
    let nr_of_problems = all_numbers_in_row[0].len();
    println!("nr_of_problems: {}", nr_of_problems);
    for i in 0..nr_of_problems {
        let mut numbers: Vec<usize> = Vec::new();
        for set in all_numbers_in_row {
            numbers.push(set[i]);
        }
        let operator = operators_in_row[i];
        let problem : Problem = Problem{
            numbers,
            operator,
        };
        problems.push(problem);
    }
    // println!("problems: {:?}", problems);
    problems
}

fn solve_problem(problem: &Problem) -> usize {
    problem.operator.apply(problem.numbers.clone())
}

fn solve_problems(problems: &Vec<Problem>) {
    let all_solutions: Vec<usize> = problems.iter().map(|problem| solve_problem(problem)).collect();
    let sum: usize = all_solutions.iter().sum();
    println!("Sum: {}", sum);
}