
pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("test_input");
    let instructions = create_list_from_input(TEST_INPUT);
    print!("instrictions: {:?}", instructions);
    let solution = count_zero(instructions);
    print!("{}", solution);
    // calc_distance(list1, list2);
    // calc_similarity(list1, list2);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction{
    direction: Direction,
    amount: i32,
}

fn create_list_from_input(input: &str) -> Vec<Instruction> {
    let mut list: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let (direction, amount) = line.split_at(1);
        let direction =
        if direction.contains("L"){
            Direction::Left
        }
        else{
            Direction::Right
        };
        list.push(Instruction{direction, amount: amount.parse().unwrap() });
    }
    list
}

fn rotate_dial(current_pos: i32, instruction: Instruction) -> (bool, i32) {
    let mut on_zero: bool = false;
    let amount_stripped = instruction.amount % 100;
    match instruction.direction {
        Direction::Left => {
            let mut next_pos = current_pos - amount_stripped;
            if next_pos <= 0 {
                next_pos  = 100 + next_pos;
            }
            if next_pos == 0{
                on_zero = true;
            }
            (on_zero, next_pos)
        }
        Direction::Right => {
            let mut next_pos = current_pos + amount_stripped;
            if next_pos >= 100 {
                next_pos = next_pos - 100;
            }
            if next_pos == 0{
                on_zero = true;
            }
            (on_zero, next_pos)
        }
    }
}

fn count_zero(instructions: Vec<Instruction>) -> u32{
    let mut count = 0;
    let mut pos = 0;
    for instruction in instructions {
        let state = rotate_dial(pos, instruction);
        pos = state.1;
        count = count + state.0 as u32;
    }
    count
}