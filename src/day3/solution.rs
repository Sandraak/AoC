pub fn solve() {
    const INPUT: &str = include_str!("./puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");

    calc_sum_of_mult(TEST_INPUT);
}

fn calc_sum_of_mult(input: &str) -> u32 {
    let mut sum_of_mult: u32 = 0;
    let mut enabled = true;

    // Collect tuples of (string before, left number, right number)
    // Collect tuples of (string before, left number, right number)
    let numbers: Vec<(&str, u32, u32)> = input
        .split("mul(")
        .enumerate()
        .filter_map(|(i, part)| {
            println!("part: '{}'", part);
            let pair = part.split_once(')')?;
            println!("part: '{}'", part);
            println!("pair.0:  '{}'", pair.0);
            // println!("before: '{}'");
            println!("pair.1:  '{}'", pair.1);
            let before = part.trim_end_matches(')').trim_end_matches(pair.0);
            println!("before: '{}'", before);
            // println!("before: {}", before);

            let (left, right) = pair.0.split_once(',')?;
            let left_num = left.parse::<u32>().ok()?;
            let right_num = right.parse::<u32>().ok()?;

            // let before = pair.1;

            // if left.len() <= 3 && right.len() <= 3 && enabled {
            //     sum_of_mult += left_num * right_num;
            //     if before.contains("don't()") && before.contains("do()") {
            //         println!(
            //             "Found both do and don't after multiplying ({},{})",
            //             left_num, right_num
            //         );
            //         enabled = false;
            //     } else if before.contains("do()") {
            //         println!(
            //             "Found 'do()' after multiplying ({},{})",
            //             left_num, right_num
            //         );
            //         enabled = true;
            //     } else if before.contains("don't()") {
            //         println!(
            //             "Found 'don't()' before multiplying ({},{})",
            //             left_num, right_num
            //         );
            //         enabled = false;
            //     }
            // }
            Some((before, left_num, right_num))
        })
        .collect();

    // for (before, left, right) in &numbers {
    //     println!("Before: '{}', Numbers: ({}, {})", before, left, right);
    // }

    println!("Sum of multiplication: {}", sum_of_mult);
    sum_of_mult
}

// fn calc_sum_of_mult(input: &str) -> u32 {
//     let mut sum_of_mult: u32 = 0;
//     let enabled = true;
//
//     let numbers: Vec<(&str, u32, u32)> = input
//         .split("mul(")
//         .enumerate()
//         .filter_map(|s| {
//             let pair = s.split_once(')').map(|(both_numbers, _)|
// both_numbers)?; //Alles tussen 'mul(' en ')'             let (left, right) =
// pair.split_once(',')?;
//
//             let left_num = left.parse::<u32>().ok()?;
//             let right_num = right.parse::<u32>().ok()?;
//
//             if left.len() <= 3 && right.len() <= 3 && enabled {
//                 sum_of_mult += left_num * right_num;
//             }
//             Some((left_num, right_num)) // Return the valid pair
//         })
//         .collect();
//     // println!("Before mul: {}", before_mul);
//     println!("Sum of multiplication: {}", sum_of_mult);
//     sum_of_mult
// }
