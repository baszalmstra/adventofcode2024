fn main() {
    let input = std::fs::read_to_string("inputs/day03/input.txt").unwrap();

    let mut enabled = true;
    let mut disabled_instructions = Vec::new();
    let mut instructions = Vec::new();
    let mut input = input.as_str();
    while !input.is_empty() {
        input = match find_instruction(input) {
            Some(("do()", rest)) => {
                enabled = true;
                rest
            }
            Some(("don't()", rest)) => {
                enabled = false;
                rest
            }
            Some(("mul(", rest)) => match find_next_operation(rest) {
                Some((operation, rest)) => {
                    if enabled {
                        instructions.push(operation);
                    } else {
                        disabled_instructions.push(operation);
                    }
                    rest
                }
                None => rest,
            },
            Some(_) => unreachable!(),
            None => break,
        }
    }

    let solution_a: u32 = instructions
        .iter()
        .chain(disabled_instructions.iter())
        .map(|(a, b)| a * b)
        .sum();
    println!("Solution 1: {}", solution_a);

    let solution_b: u32 = instructions.iter().map(|(a, b)| a * b).sum();
    println!("Solution 2: {}", solution_b);
}

fn find_next_operation(input: &str) -> Option<((u32, u32), &str)> {
    let (a, input) = scan_digits(input)?;
    let input = scan_literal(input, ",")?;
    let (b, input) = scan_digits(input)?;
    let input = scan_literal(input, ")")?;
    Some(((a, b), input))
}

fn scan_digits(input: &str) -> Option<(u32, &str)> {
    let mut num_digits = 0;
    for (offset, c) in input.char_indices() {
        if c.is_ascii_digit() {
            num_digits += 1;
        } else if num_digits > 0 {
            let digit_str = &input[..offset];
            let digits = digit_str.parse().ok()?;
            let rest = &input[offset..];
            return Some((digits, rest));
        } else {
            return None;
        }
    }

    if num_digits > 0 {
        let digits = input.parse().ok()?;
        Some((digits, ""))
    } else {
        None
    }
}

fn scan_literal<'i>(input: &'i str, literal: &str) -> Option<&'i str> {
    input.strip_prefix(literal)
}

fn find_instruction(input: &str) -> Option<(&str, &str)> {
    ["mul(", "do()", "don't()"]
        .into_iter()
        .filter_map(|literal| input.split_once(literal).map(|a| (literal, a)))
        .min_by_key(|(_, (before, _))| before.len())
        .map(|(literal, (_before, after))| (literal, after))
}
