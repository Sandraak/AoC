use std::str::FromStr;
use itertools::Itertools;

pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("test_input");
    let problems = parse_input(TEST_INPUT);
    let problems_b = parse_input_b(TEST_INPUT);
    let solution = solve_problems(&problems);
}

#[derive(Debug)]
struct Problem {
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
            _ => None,
        }
    }
}

impl FromStr for Operator {
    type Err = String; // or a custom error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}

fn parse_input(input: &str) -> Vec<Problem> {
    let mut operations: Vec<Operator> = Vec::new();
    let mut all_numbers_in_row: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        if line.contains('+') || line.contains('*') {
            operations = line
                .split_whitespace()
                .map(|operator| operator.parse::<Operator>().unwrap())
                .collect();
        } else {
            let numbers_in_row: Vec<usize> = line
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect();
            all_numbers_in_row.push(numbers_in_row);
        }
    }
    create_problems(&all_numbers_in_row, &operations)
}

struct ProblemStr {
    numbers: Vec<Vec<char>>,
    operator: Operator,
}

fn parse_input_b(input: &str)
// -> Vec<ProblemStr>
{
    let mut operations: Vec<Operator> = Vec::new();
    let mut all_numbers_in_row: Vec<Vec<Vec<char>>> = Vec::new();

    for line in input.lines() {
        println!("Line: {:?}", line);
        if line.contains('+') || line.contains('*') {
            operations = line
                .split_whitespace()
                .map(|operator| operator.parse::<Operator>().unwrap())
                .collect();
        } else {
            let mut nr: Vec<char> = Vec::new();
            let mut numbers_in_row: Vec<Vec<char>> = Vec::new();
            let line_as_vec: Vec<char> = line.chars().collect();
            let len = line_as_vec.len();
            let mut i = 0;
            while i < len {
                println!("i: {}, char: {}", i, line_as_vec[i]);

                if !in_bounds(i + 1, len) {
                    println!(
                        "the next char on pos {} is not in bounds. Adding {} to nr",
                        i + 1,
                        line_as_vec[i]
                    );
                    nr.push(line_as_vec[i]);
                    numbers_in_row.push(nr.clone());
                    nr.clear();
                }
                else if i == 0 && line_as_vec[i].is_whitespace() && line_as_vec[i +1].is_whitespace(){
                    println!("number starts with double whitespace");
                    nr.push(line_as_vec[i]);
                    nr.push(line_as_vec[i+1]);
                    println!("number after adding double shitespace: {:?}", nr);
                    i += 1;
                }
                else if line_as_vec[i + 1].is_whitespace() && !line_as_vec[i + 2].is_whitespace() {
                    println!("next char is a whitespace and next is not on i = {}.", i);
                    nr.push(line_as_vec[i]);
                    numbers_in_row.push(nr.clone());
                    println!(
                        "Adding {} to nr. nr {:?} is now done",
                        line_as_vec[i + 1],
                        nr
                    );
                    nr.clear();
                    i += 1;
                }
                else if line_as_vec[i + 1].is_whitespace() && line_as_vec[i + 2].is_whitespace() {
                    println!(
                        "next char is a whitespace and next is as well on i = {}.",
                        i
                    );
                    nr.push(line_as_vec[i]);
                    if !nr[0].is_whitespace() || (nr[0].is_whitespace() && nr.iter().all_unique()) {
                        numbers_in_row.push(nr.clone());
                        println!(
                            "Adding {} to nr. nr {:?} is now done",
                            line_as_vec[i + 1],
                            nr
                        );
                        nr.clear();
                        nr.push(line_as_vec[i+1]);
                        i+=1;
                    }
                    i += 1;
                } else {
                    nr.push(line_as_vec[i]);
                    println!(
                        "added char {} on index: {} to nr {:?}",
                        line_as_vec[i], i, nr
                    );
                }
                i += 1;

                // Ik moet kijken of de nummers gesplits worden door 1 enkele whitespace Als dat zo is. Skip die char. en voeg nr toe aan all_numbers_inline
                // Als, gesplitst door 1 of meerde whitespaces. Skip 1 whitespace en voeg de rest toe
                // Zonder whitespace: voeg char toe aan nummer
            }
            println!("numbers in row: {:?}", numbers_in_row);
            // let numbers_in_row : Vec<usize>= line.split_whitespace().map(|number| number.parse::<usize>().unwrap()).collect();
            all_numbers_in_row.push(numbers_in_row);
        }
    }
    // create_problems(&all_numbers_in_row, &operations)
}

fn in_bounds(index: usize, len: usize) -> bool {
    index < len
}

fn create_problems(
    all_numbers_in_row: &Vec<Vec<usize>>,
    operators_in_row: &Vec<Operator>,
) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let nr_of_problems = all_numbers_in_row[0].len();
    println!("nr_of_problems: {}", nr_of_problems);
    for i in 0..nr_of_problems {
        let mut numbers: Vec<usize> = Vec::new();
        for set in all_numbers_in_row {
            numbers.push(set[i]);
        }
        let operator = operators_in_row[i];
        let problem: Problem = Problem { numbers, operator };
        // println!("problem: {:?}", problem);
        problems.push(problem);
    }
    // println!("problems: {:?}", problems);
    problems
}

fn solve_problem(problem: &Problem) -> usize {
    problem.operator.apply(problem.numbers.clone())
}

fn solve_problems(problems: &Vec<Problem>) {
    let all_solutions: Vec<usize> = problems
        .iter()
        .map(|problem| solve_problem(problem))
        .collect();
    let sum: usize = all_solutions.iter().sum();
    println!("Sum: {}", sum);
}
