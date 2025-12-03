pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");
    let parsed_input = input_to_ranges(INPUT);
    let invalid_ids = check_valid(parsed_input);
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

fn input_to_ranges(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(start, end)| Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        })
        .collect()
}

fn check_valid(ranges: Vec<Range>) -> usize {
    let mut invaled_ids: Vec<usize> = Vec::new();
    for range in ranges {
        for i in range.start..range.end {
            if is_even(i) && is_symmetrical(i) {
                invaled_ids.push(i);
            }
        }
    }
    // println!("invalid ids{:?}", invaled_ids);
    let sum = invaled_ids.into_iter().sum();
    println!("sum is {}", sum);
    sum
}

fn is_symmetrical(i: usize) -> bool {
    let i_as_str = i.to_string();
    let mid = i_as_str.len() / 2;
    let (lhs, rhs) = i_as_str.split_at(mid);
    lhs == rhs
}

fn is_even(i: usize) -> bool {
    i.to_string().len() % 2 == 0
}
