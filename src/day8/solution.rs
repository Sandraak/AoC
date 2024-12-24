use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve() {
    const INPUT: &str = include_str!("./puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");
    let map = create_map(INPUT);
    let antennas = find_antennas(&map);
    let nr_of_antinodes = place_antinodes(antennas, &map);
    print!("Nr of antinodes: {}", nr_of_antinodes);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Pos {
    row: isize,
    col: isize,
}

#[derive(Debug)]
struct Antenna {
    frequency: char,
    pos: Pos,
}

fn create_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn place_antinodes(antennas: Vec<Antenna>, map: &Vec<Vec<char>>) -> u64 {
    let antennas_hashmap: HashMap<char, Vec<Antenna>> = antennas
        .into_iter()
        .into_grouping_map_by(|antenna| antenna.frequency)
        .collect();
    // let connections = find_connections(antennas_hashmap, map);
    let connections = find_connections_with_resonant_harmonics(antennas_hashmap, map);
    // let mut new_map = map.clone();
    // for connection in &connections {
    //     new_map[connection.row as usize][connection.col as usize] = '#';
    // }
    // print_map(&new_map);
    connections.len() as u64
}

fn find_connections(
    antennas_hashmap: HashMap<char, Vec<Antenna>>,
    map: &Vec<Vec<char>>,
) -> Vec<Pos> {
    let mut connections = Vec::new();
    antennas_hashmap.values().for_each(|antennas| {
        for antenna in antennas {
            for other_antenna in antennas {
                if antenna.pos != other_antenna.pos {
                    if let Some(connection) =
                        get_antinode_pos(&antenna.pos, &other_antenna.pos, map)
                    {
                        {
                            if !connections.contains(&connection) {
                                connections.push(connection);
                            }
                        }
                    }
                }
            }
        }
    });
    connections
}

fn find_connections_with_resonant_harmonics(
    antennas_hashmap: HashMap<char, Vec<Antenna>>,
    map: &Vec<Vec<char>>,
) -> Vec<Pos> {
    let mut connections = HashSet::new();
    antennas_hashmap.values().for_each(|antennas| {
        for antenna in antennas {
            for other_antenna in antennas {
                if antenna.pos != other_antenna.pos {
                    let new_connections = get_antinodes_with_resonant_harmonics(
                        &antenna.pos,
                        &other_antenna.pos,
                        map,
                    );
                    for connection in new_connections {
                        connections.insert(connection);
                    }
                }
            }
        }
    });
    connections.into_iter().collect()
}

fn get_antinodes_with_resonant_harmonics(
    current_antenna: &Pos,
    other_antenna: &Pos,
    map: &Vec<Vec<char>>,
) -> Vec<Pos> {
    let mut antinodes = vec![];
    let width = map[0].len();
    let height = map.len();
    let delta_x = other_antenna.col - current_antenna.col;
    let delta_y = other_antenna.row - current_antenna.row;
    let mut antinode_pos = Pos {
        row: other_antenna.row,
        col: other_antenna.col,
    };
    while (antinode_pos.row < height as isize && antinode_pos.col < width as isize)
        && (antinode_pos.row >= 0 && antinode_pos.col >= 0)
    {
        antinodes.push(antinode_pos);
        antinode_pos = Pos {
            row: antinode_pos.row + delta_y,
            col: antinode_pos.col + delta_x,
        };
    }
    antinodes
}

fn get_antinode_pos(
    current_antenna: &Pos,
    other_antenna: &Pos,
    map: &Vec<Vec<char>>,
) -> Option<Pos> {
    let width = map[0].len();
    let height = map.len();
    let antinode_pos = Pos {
        row: 2 * current_antenna.row - other_antenna.row,
        col: 2 * current_antenna.col - other_antenna.col,
    };
    if (antinode_pos.row < 0 || antinode_pos.col < 0)
        || (antinode_pos.row >= height as isize || antinode_pos.col >= width as isize)
    {
        return None;
    }
    Some(antinode_pos)
}

fn find_antennas(map: &Vec<Vec<char>>) -> Vec<Antenna> {
    let mut antennas = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] != '.' {
                antennas.push(Antenna {
                    frequency: map[row][col],
                    pos: Pos {
                        row: row as isize,
                        col: col as isize,
                    },
                });
            }
        }
    }
    antennas
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
