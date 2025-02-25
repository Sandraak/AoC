pub fn solve() {
    const INPUT: &str = include_str!("./puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");
    let input = parse_input(TEST_INPUT);
    println!("input: {:?}", input);
    visualize(input);
}

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn visualize(diskmap: Vec<char>) -> Vec<char> {
    let mut blocks: Vec<char> = Vec::new();
    let mut id: u8 = 0;
    for i in 0..diskmap.len() {
        let mut input = '.';
        if i % 2 == 0 {
            input = id as char;
            id += 1;
            println!("input: {}", input);
        }
        let nr_of_blocks = diskmap[i] as u8 - 48;
        println!("nr_of_blocks: {} on i = {}", nr_of_blocks, i);
        for j in 0..nr_of_blocks {
            blocks.push(input);
        }
    }
    println!("{:?}", blocks);
    blocks
}
