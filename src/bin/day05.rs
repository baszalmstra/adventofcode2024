use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day05/input.txt").expect("Can't read data file");

    let mut lines = input.lines();
    let mut rules: HashMap<_, Vec<_>> = HashMap::new();
    let mut rev_rules: HashMap<_, Vec<_>> = HashMap::new();
    loop {
        let Some(line) = lines.next() else { break };
        if line.is_empty() {
            break;
        }

        let (left, right) = line.split_once('|').expect("Invalid rule");
        let left = left.trim().parse::<u32>().expect("Invalid rule number");
        let right = right.trim().parse::<u32>().expect("Invalid rule number");

        rules.entry(left).or_default().push(right);
        rev_rules.entry(right).or_default().push(left);
    }

    let mut updates = Vec::new();
    loop {
        let Some(line) = lines.next() else { break };
        if line.is_empty() {
            break;
        }

        updates.push(line.split(',')
            .map(|c| c.parse().expect("Invalid number"))
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<HashMap<_, _>>());
    }

    let mut score = 0;
    let mut score_b = 0;
    for mut update in updates {
        let mut is_valid = true;
        let mut middle = None;
        let middle_idx = update.len() / 2;
        for (&item, &idx) in &update {
            if idx == middle_idx {
                middle = Some(item);
            }
            for right in rules.get(&item).into_iter().flatten() {
                if update.get(right).map_or(false, |n| *n < idx) {
                    is_valid = false;
                    break;
                }
            }
            if is_valid == false {
                break;
            }
        }

        if is_valid {
            score += middle.unwrap();
        } else {
            let mut ordered = Vec::new();
            while !update.is_empty() {
                let ordered_key = *update.keys().find(|(&value)| {
                    let Some(rule) = rev_rules.get(value) else { return true };
                    for rule in rule {
                        if update.contains_key(rule){
                            return false;
                        }
                    }
                    true
                }).expect("no key found");
                update.remove(&ordered_key);
                ordered.push(ordered_key);
            }
            score_b += ordered[ordered.len() / 2];
        }
    }

    println!("Solution 1: {}", score);
    println!("Solution 2: {}", score_b);
}
