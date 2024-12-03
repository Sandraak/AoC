pub fn solve() {
    const INPUT: &str = include_str!("./puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");

    calc_sum_of_mult(INPUT);
}

fn calc_sum_of_mult(input: &str) -> u32 {
    let mut sum_of_mult: u32 = 0;
    let enabled = true;

    let numbers: Vec<(u32, u32)> = input
        .split("mul(")
        .skip(1) // Skip any text before the first "mul("
        .filter_map(|s| {
            let pair = s.split_once(')').map(|(both_numbers, _)| both_numbers)?; //Alles tussen 'mul(' en ')'
            let (left, right) = pair.split_once(',')?;

            let left_num = left.parse::<u32>().ok()?;
            let right_num = right.parse::<u32>().ok()?;

            if left.len() <= 3 && right.len() <= 3 && enabled {
                sum_of_mult += left_num * right_num;
            }
            Some((left_num, right_num)) // Return the valid pair
        })
        .collect();

    println!("Sum of multiplication: {}", sum_of_mult);
    sum_of_mult
}
