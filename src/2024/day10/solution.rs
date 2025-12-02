use std::collections::HashSet;

pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("test_input");
    let map = input_to_map(INPUT);
    let trailheads = find_trailheads(&map);
    // let score = trailheads
    //     .into_iter()
    //     .map(|trailhead| determine_score(trailhead, &map).len())
    //     .sum::<usize>();
    // println!("Score: {}", score);
    let rating = trailheads
        .into_iter()
        .map(|trailhead| determine_rating(trailhead, &map))
        .sum::<u32>();
    println!("Rating: {:?}", rating);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

fn input_to_map(input: &str) -> Vec<Vec<usize>> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    map
}

fn find_trailheads(map: &[Vec<usize>]) -> Vec<Pos> {
    let mut trailheads = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 0 {
                trailheads.push(Pos { row, col });
            }
        }
    }
    trailheads
}

fn determine_score(trailhead: Pos, map: &[Vec<usize>]) -> HashSet<Pos> {
    let current_pos = trailhead;
    let map_width = map[0].len();
    let map_height = map.len();
    let possible_locations = check_possible_locations(map, current_pos, map_width, map_height);
    if map[trailhead.row][trailhead.col] == 9 {
        Some(trailhead).into_iter().collect()
    } else {
        let mut peaks = HashSet::new();
        for pos in possible_locations {
            peaks = peaks.into_iter().chain(determine_score(pos, map)).collect();
        }
        peaks
    }
}

fn determine_rating(trailhead: Pos, map: &[Vec<usize>]) -> u32 {
    let current_pos = trailhead;
    let map_width = map[0].len();
    let map_height = map.len();
    let possible_locations = check_possible_locations(map, current_pos, map_width, map_height);
    if map[trailhead.row][trailhead.col] == 9 {
        1
    } else {
        let mut rating = 0;
        for pos in possible_locations {
            rating += determine_rating(pos, map);
        }
        rating
    }
}

fn check_possible_locations(
    map: &[Vec<usize>],
    current_pos: Pos,
    map_width: usize,
    map_height: usize,
) -> Vec<Pos> {
    let mut possible_locations = Vec::new();

    let directions = [
        (-1, 0), // Up
        (1, 0),  // Down
        (0, -1), // Left
        (0, 1),  // Right
    ];

    for &(dr, dc) in &directions {
        let new_row = current_pos.row as isize + dr;
        let new_col = current_pos.col as isize + dc;

        if new_row >= 0
            && new_row < map_height as isize
            && new_col >= 0
            && new_col < map_width as isize
            && map[new_row as usize][new_col as usize] == map[current_pos.row][current_pos.col] + 1
        {
            possible_locations.push(Pos {
                row: new_row as usize,
                col: new_col as usize,
            });
        }
    }
    possible_locations
}

fn print_map(map: &Vec<Vec<usize>>) {
    for row in map {
        println!("{:?}", row);
    }
}
