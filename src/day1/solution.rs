pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("test_input");
    let instructions = create_list_from_input(INPUT);
    let solution = count_zero(instructions);
    println!("count {}", solution);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

#[derive(Debug)]
struct State {
    zero_count: u32,
    pos: i32,
}

fn create_list_from_input(input: &str) -> Vec<Instruction> {
    let mut list: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let (direction, amount) = line.split_at(1);
        let direction = if direction.contains("L") {
            Direction::Left
        } else {
            Direction::Right
        };
        list.push(Instruction {
            direction,
            amount: amount.parse().unwrap(),
        });
    }
    list
}

fn rotate_dial(current_pos: i32, instruction: Instruction) -> State {
    let mut on_zero: u32 = 0;
    on_zero += instruction.amount as u32 / 100;
    let amount_stripped = instruction.amount % 100;

    match instruction.direction {
        Direction::Left => {
            let mut next_pos = current_pos - amount_stripped;
            if next_pos < 0 {
                if current_pos != 0 {
                    on_zero += 1;
                }
                next_pos = 100 + next_pos;
            }
            if next_pos == 0 || next_pos == 100 {
                on_zero += 1;
            }
            State {
                zero_count: on_zero,
                pos: next_pos,
            }
        }
        Direction::Right => {
            let mut next_pos = current_pos + amount_stripped;
            if next_pos > 100 {
                if current_pos != 100 {
                    on_zero += 1;
                }
                next_pos = next_pos - 100;
            }
            if next_pos == 0 || next_pos == 100 {
                on_zero += 1;
            }
            State {
                zero_count: on_zero,
                pos: next_pos,
            }
        }
    }
}

fn count_zero(instructions: Vec<Instruction>) -> u32 {
    let mut count = 0;
    let mut pos = 50;
    for instruction in instructions {
        let state = rotate_dial(pos, instruction);
        pos = state.pos;
        count = count + state.zero_count;
    }
    count
}
