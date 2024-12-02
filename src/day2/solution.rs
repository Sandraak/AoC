pub fn solve() {
    const INPUT: &str = include_str!("./puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");
    let nr_of_safe_reports = calc_nr_of_safe_reports(INPUT);
    println!("Number of safe reports: {}", nr_of_safe_reports);
}

fn calc_nr_of_safe_reports(input: &str) -> u32 {
    let mut nr_of_safe_reports = 0;
    for report in input.lines() {
        let report = report
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<Vec<_>>();

        if is_safe(&report) || is_safe_by_removing_one(&report) {
            nr_of_safe_reports += 1;
        }
    }
    nr_of_safe_reports
}

fn is_safe(report: &[u32]) -> bool {
    let is_sorted = report.iter().is_sorted_by(|a, b| a > b) || report.iter().is_sorted_by(|a, b| a < b);
    let valid_differences = report.windows(2).all(|pair| {
        let diff = (pair[0] as i32 - pair[1] as i32).abs();
        (1..=3).contains(&diff)
    });
    is_sorted && valid_differences
}

fn is_safe_by_removing_one(report: &[u32]) -> bool {
    (0..report.len()).any(|i| is_safe(&report.iter().enumerate()
        .filter_map(|(j, &val)| if j != i { Some(val) } else { None })
        .collect::<Vec<_>>()))
}