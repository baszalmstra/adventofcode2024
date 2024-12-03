use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("inputs/day02/input.txt").unwrap();

    let reports = input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let solution_a = reports
        .iter()
        .filter(|report| is_valid_report(report).is_ok())
        .count();
    println!("Solution 1: {}", solution_a);

    let solution_b = reports
        .iter()
        .filter(|report| is_valid_report_with_dampen(report))
        .count();
    println!("Solution 2: {}", solution_b);
}

fn is_valid_report_with_dampen(report: &[i32]) -> bool {
    let idx = match is_valid_report(report) {
        Ok(()) => return true,
        Err(idx) => idx,
    };

    for offset in -1..=1 {
        let mut removed = report.to_vec();
        let idx = idx as isize + offset;
        if idx < 0 || idx >= report.len() as isize {
            continue;
        }
        removed.remove(idx as usize);
        if is_valid_report(&removed).is_ok() {
            return true;
        }
    }

    false
}

fn is_valid_report(report: &[i32]) -> Result<(), usize> {
    let mut delta: Option<i32> = None;
    for (idx, (&a, &b)) in report.iter().tuple_windows().enumerate() {
        let current_delta = b - a;
        match (current_delta, delta) {
            (a, _) if a.abs() < 1 || a.abs() > 3 => return Err(idx),
            (a, Some(b)) if a.signum() != b.signum() => return Err(idx),
            _ => {
                delta = Some(current_delta);
            }
        }
    }
    Ok(())
}
