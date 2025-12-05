pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("test_input");
    let batteries = input_to_battery_banks(INPUT);
    let solution = calculate_joltage(batteries);
}

#[derive(Debug)]
struct Battery {
    index: usize,
    joltage: u32,
}

const NR_OF_BATTERIES: usize = 12;

fn input_to_battery_banks(input: &str) -> Vec<Vec<u32>> {
    let mut batteries: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let mut bank: Vec<u32> = Vec::new();
        for char in line.chars() {
            bank.push(char.to_digit(10).unwrap());
        }
        batteries.push(bank.clone());
    }
    batteries
}

fn calculate_joltage(batteries: Vec<Vec<u32>>) -> u128 {
    let mut sum_of_joltage = 0;
    for bank in batteries {
        sum_of_joltage += find_optimal_combination(bank);
    }
    println!("Sum of joltage: {}", sum_of_joltage);
    sum_of_joltage
}

const MULTIPLIER: [u128; 12] = [
    100000000000,
    10000000000,
    1000000000,
    100000000,
    10000000,
    1000000,
    100000,
    10000,
    1000,
    100,
    10,
    1,
];

fn find_optimal_combination(bank: Vec<u32>) -> u128 {
    let mut start_index = 0;
    let mut nr_of_batteries_remaining = 12;
    let mut selected_battery_joltage: Vec<u32> = Vec::new();
    for _ in 0..NR_OF_BATTERIES {
        let mut selected_battery = Battery {
            index: 0,
            joltage: 0,
        };
        for i in start_index..bank.len() - nr_of_batteries_remaining + 1 {
            if bank[i] > selected_battery.joltage {
                selected_battery.joltage = bank[i];
                selected_battery.index = i;
            }
        }
        start_index = selected_battery.index + 1;
        selected_battery_joltage.push(selected_battery.joltage);
        nr_of_batteries_remaining -= 1;
    }
    let summed_joltage = mash_vectors(selected_battery_joltage, MULTIPLIER)
        .into_iter()
        .sum();
    summed_joltage
}

fn mash_vectors(a: Vec<u32>, b: [u128; 12]) -> Vec<u128> {
    let mut result = Vec::new();
    for i in 0..a.len() {
        result.push(a[i] as u128 * b[i]);
    }
    result
}
