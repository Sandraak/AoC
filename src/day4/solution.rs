pub fn solve() {
    const INPUT: &str = include_str!("./puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");
    count_x_mas(INPUT);
}

fn count_x_mas(input: &str) -> u32 {
    let mut x_mas_count: u32 = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for row in 1..matrix.len() - 1 {
        for col in 1..matrix[row].len() - 1 {
            if matrix[row][col] == 'A'
                && ((matrix[row - 1][col - 1] == 'M' && matrix[row + 1][col + 1] == 'S')
                    || (matrix[row - 1][col - 1] == 'S' && matrix[row + 1][col + 1] == 'M'))
                && ((matrix[row + 1][col - 1] == 'M' && matrix[row - 1][col + 1] == 'S')
                    || (matrix[row + 1][col - 1] == 'S' && matrix[row - 1][col + 1] == 'M'))
            {
                x_mas_count += 1;
            }
        }
    }
    println!("Number of X_MAS: {}", x_mas_count);
    x_mas_count
}

fn count_xmas(input: &str) -> u32 {
    let mut xmas_count: u32 = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'X' {
                //check left
                if col >= 3
                    && matrix[row][col - 1] == 'M'
                    && matrix[row][col - 2] == 'A'
                    && matrix[row][col - 3] == 'S'
                {
                    xmas_count += 1;
                }
                //check right
                if col + 3 < matrix[row].len()
                    && matrix[row][col + 1] == 'M'
                    && matrix[row][col + 2] == 'A'
                    && matrix[row][col + 3] == 'S'
                {
                    xmas_count += 1;
                }
                //check up
                if row >= 3
                    && matrix[row - 1][col] == 'M'
                    && matrix[row - 2][col] == 'A'
                    && matrix[row - 3][col] == 'S'
                {
                    xmas_count += 1;
                }
                // check down
                if row + 3 < matrix.len()
                    && matrix[row + 1][col] == 'M'
                    && matrix[row + 2][col] == 'A'
                    && matrix[row + 3][col] == 'S'
                {
                    xmas_count += 1;
                }
                // check up-left
                if row >= 3
                    && col >= 3
                    && matrix[row - 1][col - 1] == 'M'
                    && matrix[row - 2][col - 2] == 'A'
                    && matrix[row - 3][col - 3] == 'S'
                {
                    xmas_count += 1;
                }
                // check up-right
                if row >= 3
                    && col + 3 < matrix[row].len()
                    && matrix[row - 1][col + 1] == 'M'
                    && matrix[row - 2][col + 2] == 'A'
                    && matrix[row - 3][col + 3] == 'S'
                {
                    xmas_count += 1;
                }
                // check down-left
                if row + 3 < matrix.len()
                    && col >= 3
                    && matrix[row + 1][col - 1] == 'M'
                    && matrix[row + 2][col - 2] == 'A'
                    && matrix[row + 3][col - 3] == 'S'
                {
                    xmas_count += 1;
                }
                // check down-right
                if col + 3 < matrix[row].len()
                    && row + 3 < matrix.len()
                    && matrix[row + 1][col + 1] == 'M'
                    && matrix[row + 2][col + 2] == 'A'
                    && matrix[row + 3][col + 3] == 'S'
                {
                    xmas_count += 1;
                }
            }
        }
    }
    println!("Number of XMAS: {}", xmas_count);
    xmas_count
}
