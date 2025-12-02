use std::collections::BTreeMap;

pub fn solve() {
    const INPUT: &str = include_str!("puzzle_input");
    const TEST_INPUT: &str = include_str!("test_input");
    let (list1, list2) = create_lists_from_input(INPUT);
    // calc_distance(list1, list2);
    calc_similarity(list1, list2);
}

fn create_lists_from_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        list1.push(left.parse().unwrap());
        list2.push(right.parse().unwrap());
    }
    list1.sort_by(|a, b| b.cmp(a));
    list2.sort_by(|a, b| b.cmp(a));

    (list1, list2)
}

fn calc_distance(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    let mut distance = 0;
    for i in 0..list1.len() {
        distance += list1[i].abs_diff(list2[i]);
    }
    println!("Distance: {}", distance);
    distance
}

fn calc_similarity(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    let mut similarity = 0;

    let mut hashmap_list2: BTreeMap<u32, u32> = BTreeMap::new();
    for i in list2 {
        if hashmap_list2.contains_key(&i) {
            *hashmap_list2.get_mut(&i).unwrap() += 1;
        } else {
            hashmap_list2.insert(i, 1);
        }
    }

    for i in list1 {
        if hashmap_list2.contains_key(&i) {
            similarity += (i * hashmap_list2.get(&i).unwrap());
        }
    }
    println!("Similarity: {}", similarity);
    similarity
}

