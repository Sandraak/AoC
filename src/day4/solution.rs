pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("test_input");
    let grid = parse_input(INPUT);
    print_grid(&grid);
    println!("\n ================ \n");
    let new_grid = mark_accessible_paper(grid);
    print_grid(&new_grid);
    let nr_of_accessible_papers = count_accessible_papers(&new_grid);
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn mark_accessible_paper(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = grid.clone();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let pos = Pos { x: row, y: col };
            if grid[row][col] == '@' && check_accessible(&grid, &pos) {
                new_grid[row][col] = 'x';
            }
        }
    }
    new_grid
}

fn count_accessible_papers(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row in grid {
        count += row.iter().filter(|&c| *c == 'x').count();
    }
    println!("Number of accessible papers: {}", count);
    count
}

fn check_accessible(grid: &Vec<Vec<char>>, pos: &Pos) -> bool {
    let mut result = Vec::new();
    let height = grid.len();
    let width = grid[0].len();

    for (dx, dy) in DIRECTIONS {
        let nx = pos.x as isize + dx;
        let ny = pos.y as isize + dy;

        if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
            result.push(grid[nx as usize][ny as usize]);
        }
    }
    result.iter().filter(|&c| *c == '@').count() < 4
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    grid
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{:?}", row);
    }
}
