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

fn input_to_battery_banks(input: &str) -> Vec<Vec<u32>> {
    let mut batteries: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        let mut bank: Vec<u32> = Vec::new();
        for char in line.chars() {
            bank.push(char.to_digit(10).unwrap());
        }
        // println!("{:?}", &bank);
        batteries.push(bank.clone());
    }
    batteries
}

fn calculate_joltage(batteries: Vec<Vec<u32>>) -> u32 {
    let mut sum_of_joltage = 0;
    for bank in batteries {
        sum_of_joltage += find_optimal_combination(bank);
    }
    println!("Sum of joltage: {}", sum_of_joltage);
    sum_of_joltage
}

fn find_optimal_combination(bank: Vec<u32>) -> u32 {
    let mut first_battery: Battery = Battery {
        index: 0,
        joltage: 0,
    };
    let mut second_battery: Battery = Battery {
        index: 0,
        joltage: 0,
    };
    for i in 0..bank.len() -1 {
        if bank[i] > first_battery.joltage {
            first_battery = Battery {
                index: i,
                joltage: bank[i],
            };
        }
    }
    for j in first_battery.index + 1..bank.len() {
        if bank[j] > second_battery.joltage {
            second_battery = Battery {
                index: j,
                joltage: bank[j],
            };
        }
    }
    // println!(
    //     "first battery: {:?}, second battery: {:?} ",
    //     &first_battery, &second_battery
    // );
    let joltage = (first_battery.joltage * 10) + second_battery.joltage;
    // println!("joltage: {}", joltage);
    joltage
}
