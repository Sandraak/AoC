use std::cmp::Ordering::*;

pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("test_input");
    let (ids, ranges) = parse_input(INPUT);
    let merged_ranges = merge_ranges(ranges);
    // let (smallest, largest) = get_smallest_and_largest_start_range(&merged_ranges);
    // let all_ids = all_ids_in_range(&ranges);
    // let all_possible_fresh_ids = remove_rotten_ids(smallest, largest,ids);
    // let fresh_ingredients = fresh_ingredients(all_possible_fresh_ids, &merged_ranges);
    let count = calc_nr_of_fresh_ingredients(&merged_ranges);
}

#[derive(Debug)]
struct Range {
    start: u128,
    end: u128,
}

fn calc_nr_of_fresh_ingredients(ranges: &[Range]) -> u128 {
    let mut count = 0;
    for range in ranges {
        count += (range.end - range.start + 1);
    }
    println!("nr of fresh ingredients: {}", count);
    count
}

fn fresh_ingredients(ids: Vec<u128>, fresh_ranges: &Vec<Range>) -> Vec<u128> {
    let mut fresh_ingredients: Vec<u128> = Vec::new();
    for id in ids {
        if is_in_ranges(id, fresh_ranges) {
            fresh_ingredients.push(id);
        }
    }
    println!("nr of fresh ingredients: {}", fresh_ingredients.len());
    fresh_ingredients
}

fn is_in_ranges(id: u128, ranges: &Vec<Range>) -> bool {
    match ranges.binary_search_by(|range| {
        if id < range.start {
            Greater
        } else if id > range.end {
            Less
        } else {
            Equal
        }
    }) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/*
    * 3-5
    *10-14
    *12-18
    *16-20

    *=> 3-5, 10-20
*/
fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|r| r.start);
    let mut merged: Vec<Range> = Vec::new();

    for range in ranges {
        if let Some(last_range) = merged.last_mut() {
            if range.start <= last_range.end + 1 {
                last_range.end = last_range.end.max(range.end);
                continue;
            }
        }
        merged.push(range);
    }
    merged
}

fn get_smallest_and_largest_start_range(ranges: &Vec<Range>) -> (u128, u128) {
    let smallest = ranges.iter().map(|range| range.start).min().unwrap();
    let biggest = ranges.iter().map(|range| range.end).max().unwrap();
    (smallest, biggest)
}

fn remove_rotten_ids(smallest: u128, largest: u128, ids: Vec<u128>) -> Vec<u128> {
    let mut optimized_ids = Vec::new();
    for id in ids {
        if id > smallest && id < largest {
            optimized_ids.push(id);
        }
    }
    optimized_ids
}

fn parse_input(input: &str) -> (Vec<u128>, Vec<Range>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut ids: Vec<u128> = parts[1]
        .lines()
        .map(|l| l.parse::<u128>().unwrap())
        .collect();
    ids.sort();
    let ranges: Vec<Range> = input_to_ranges(parts[0]);
    (ids, ranges)
}

fn input_to_ranges(input: &str) -> Vec<Range> {
    input
        .split("\n")
        .map(|range| range.split_once('-').unwrap())
        .map(|(start, end)| Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        })
        .collect()
}
