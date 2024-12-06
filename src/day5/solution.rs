pub fn solve() {
    const INPUT: &str = include_str!("./puzzle_input");
    const TEST_INPUT: &str = include_str!("./test_input");
    let (rules, reports) = get_rules_and_reports(INPUT);
    let (valid_reports, invalid_reports) = get_valid_and_invalid_reports(&rules, reports);
    // let sum_valid = sum_of_middle_pages(valid_reports);
    let corrected_reports = correct_invalid_reports(invalid_reports, &rules);
    let sum_corrected = sum_of_middle_pages(corrected_reports);
}

fn get_rules_and_reports(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules: Vec<(u32, u32)> = vec![];
    let mut reports: Vec<Vec<u32>> = vec![];
    for rule in input.lines() {
        if rule.contains("|") {
            let rule: (u32, u32) = (rule.split_once("|"))
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();
            rules.push(rule);
        } else if rule.contains(",") {
            let string_reports: Vec<&str> = rule.split(",").collect();
            let report: Vec<u32> = string_reports
                .iter()
                .map(|report| report.parse().unwrap())
                .collect();
            reports.push(report);
        }
    }
    (rules, reports)
}

fn sum_of_middle_pages(reports: Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;
    for report in reports {
        let middle_page = report[report.len() / 2];
        sum += middle_page;
    }
    println!("Sum of middle pages: {}", sum);
    sum
}

fn is_correct(report: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    let mut checked_numbers: Vec<u32> = vec![];
    for number_index in 0..report.len() {
        if rules
            .iter()
            .any(|(a, b)| report[number_index] == *a || report[number_index] == *b)
            && !checked_numbers.contains(&report[number_index])
        {
            let relevant_rules = rules
                .iter()
                .filter(|(a, b)| report[number_index] == *a || report[number_index] == *b)
                .collect::<Vec<&(u32, u32)>>();
            for rule in relevant_rules {
                if rule.0 == report[number_index] {
                    for i in 0..number_index {
                        if report[i] == rule.1 {
                            return false;
                        }
                    }
                } else if rule.1 == report[number_index] {
                    for i in number_index..report.len() {
                        if report[i] == rule.0 {
                            return false;
                        }
                    }
                }
            }
            checked_numbers.push(report[number_index]);
        }
    }
    true
}

fn correct_report(report: Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut checked_numbers: Vec<u32> = vec![];
    let mut corrected_report = report.clone();
    for number_index in 0..corrected_report.len() {
        if rules.iter().any(|(a, b)| {
            corrected_report[number_index] == *a || corrected_report[number_index] == *b
        }) && !checked_numbers.contains(&corrected_report[number_index])
        {
            let relevant_rules = rules
                .iter()
                .filter(|(a, b)| {
                    corrected_report[number_index] == *a || corrected_report[number_index] == *b
                })
                .collect::<Vec<&(u32, u32)>>();
            for rule in relevant_rules {
                if rule.0 == corrected_report[number_index] {
                    for i in 0..number_index {
                        if corrected_report[i] == rule.1 {
                            corrected_report.swap(i, number_index);
                        }
                    }
                } else if rule.1 == corrected_report[number_index] {
                    for i in number_index..corrected_report.len() {
                        if corrected_report[i] == rule.0 {
                            corrected_report.swap(i, number_index);
                        }
                    }
                }
            }
        }
        checked_numbers.push(corrected_report[number_index]);
    }
    if !is_correct(&corrected_report, rules) {
        return correct_report(corrected_report.clone(), rules);
    }
    corrected_report
}

fn correct_invalid_reports(
    invalid_reports: Vec<Vec<u32>>,
    rules: &Vec<(u32, u32)>,
) -> Vec<Vec<u32>> {
    let mut corrected_reports: Vec<Vec<u32>> = vec![];
    for report in invalid_reports {
        corrected_reports.push(correct_report(report, &rules));
    }
    corrected_reports
}

fn get_valid_and_invalid_reports(
    rules: &Vec<(u32, u32)>,
    reports: Vec<Vec<u32>>,
) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut valid_reports: Vec<Vec<u32>> = vec![];
    let mut invalid_reports: Vec<Vec<u32>> = vec![];
    for report in reports {
        if is_correct(&report, &rules) {
            valid_reports.push(report);
        } else {
            invalid_reports.push(report);
        }
    }
    (valid_reports, invalid_reports)
}
