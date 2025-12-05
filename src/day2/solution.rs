use itertools::Itertools;

pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");
    let parsed_input = input_to_ranges(INPUT);
    let invalid_ids = split_in_equal_pieces_and_check_if_valid(parsed_input);
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

fn split_in_equal_pieces_and_check_if_valid(ranges: Vec<Range>) -> usize {
    let mut invalid_ids: Vec<usize> = Vec::new();
    let prime_numbers = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    for range in ranges {
        for i in range.start..=range.end {
            let i_as_str = i.to_string();
            let i_len = i_as_str.len();
            for prime_number in prime_numbers {
                if i_len % prime_number == 0 {
                    let chunks = i_as_str
                        .chars()
                        .chunks(i_len / prime_number)
                        .into_iter()
                        .map(|chunk| chunk.into_iter().map(|c| c.to_string()).collect::<String>())
                        .collect::<Vec<String>>();
                    if all_same(&chunks) {
                        invalid_ids.push(i);
                        break;
                    }
                }
            }
        }
    }
    let sum = invalid_ids.into_iter().sum();
    println!("sum is {}", sum);
    sum
}
fn all_same<T: PartialEq>(v: &[T]) -> bool {
    if v.len() < 2 {
        return true;
    }
    v.iter().all(|x| x == &v[0])
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
