pub fn solve() {
    const INPUT: &str = include_str!("./puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");
    let (map, guard_pos) = find_guard(INPUT);
    find_path(map, guard_pos);
}

#[derive(Debug)]
struct Guard {
    pos: Pos,
    direction: Direction,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    col: usize,
    row: usize,
}

fn find_guard(input: &str) -> (Vec<Vec<char>>, Guard) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut guard = Guard {
        pos: Pos { col: 0, row: 0 },
        direction: Direction::Up,
    };
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == '^' {
                guard = Guard {
                    pos: Pos { col, row },
                    direction: Direction::Up,
                };
            }
            if map[row][col] == 'v' {
                guard = Guard {
                    pos: Pos { col, row },
                    direction: Direction::Down,
                };
            }
            if map[row][col] == '<' {
                guard = Guard {
                    pos: Pos { col, row },
                    direction: Direction::Left,
                };
            }
            if map[row][col] == '>' {
                guard = Guard {
                    pos: Pos { col, row },
                    direction: Direction::Right,
                };
            }
        }
    }
    println!("guard: {:?}", guard);
    print_map(map.clone());
    (map, guard)
}

impl Guard {
    fn step(&mut self) {
        match self.direction {
            Direction::Up => {
                self.pos.row -= 1;
            }
            Direction::Down => {
                self.pos.row += 1;
            }
            Direction::Left => {
                self.pos.col -= 1;
            }
            Direction::Right => {
                self.pos.col += 1;
            }
        }
    }
}

fn determine_direction(guard: &Guard, map: &Vec<Vec<char>>) -> Direction {
    match guard.direction {
        Direction::Up => {
            if map[guard.pos.row - 1][guard.pos.col] == '#' {
                Direction::Right
            } else {
                Direction::Up
            }
        }
        Direction::Down => {
            if map[guard.pos.row + 1][guard.pos.col] == '#' {
                Direction::Left
            } else {
                Direction::Down
            }
        }
        Direction::Left => {
            if map[guard.pos.row][guard.pos.col - 1] == '#' {
                Direction::Up
            } else {
                Direction::Left
            }
        }
        Direction::Right => {
            if map[guard.pos.row][guard.pos.col + 1] == '#' {
                Direction::Down
            } else {
                Direction::Right
            }
        }
    }
}

fn guard_left_map(guard: &Guard, map: &Vec<Vec<char>>) -> bool {
    // let mut guard_left :bool = false;
    match guard.direction {
        Direction::Up => guard.pos.row == 0,
        Direction::Down => guard.pos.row == map.len() - 1,
        Direction::Left => guard.pos.col == 0,
        Direction::Right => guard.pos.col == map[0].len() - 1,
    }
}

fn find_path(map: Vec<Vec<char>>, mut guard: Guard) {
    let mut map_with_path = map.clone();
    loop {
        map_with_path[guard.pos.row][guard.pos.col] = 'X';
        if guard_left_map(&guard, &map) {
            break;
        }
        guard.direction = determine_direction(&guard, &map);
        guard.step();
    }
    let steps = count_steps(&map_with_path);
    // print_map(map_with_path);
    println!("steps: {}", steps);
}

fn count_steps(map: &Vec<Vec<char>>) -> usize {
    let mut steps = 0;
    for row in map {
        for c in row {
            if *c == 'X' {
                steps += 1;
            }
        }
    }
    steps
}

fn print_map(map: Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
