use itertools::Itertools;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const MAS: [char; 3] = ['M', 'A', 'S'];
const SAM: [char; 3] = ['S', 'A', 'M'];

fn main() {
    let input = std::fs::read_to_string("inputs/day04/input.txt").unwrap();

    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            for (dir_x, dir_y) in [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ]
            .iter()
            {
                if has_xmas(x as i32, y as i32, *dir_x, *dir_y, &grid, &XMAS) {
                    count += 1;
                }
            }
        }
    }

    println!("Solution 1: {}", count);

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let x = x as i32;
            let y = y as i32;
            let found = (has_xmas(x, y, 1, 1, &grid, &MAS) || has_xmas(x, y, 1, 1, &grid, &SAM))
                && (has_xmas(x + 2, y, -1, 1, &grid, &MAS)
                    || has_xmas(x + 2, y, -1, 1, &grid, &SAM));
            if found {
                count += 1;
            }
        }
    }

    println!("Solution 2: {}", count);
}

fn has_xmas(
    mut pos_x: i32,
    mut pos_y: i32,
    dir_x: i32,
    dir_y: i32,
    grid: &[Vec<char>],
    word: &[char],
) -> bool {
    for &c in word.iter() {
        if pos_x < 0
            || pos_y < 0
            || pos_y as usize >= grid.len()
            || pos_x as usize >= grid[pos_y as usize].len()
            || grid[pos_y as usize][pos_x as usize] != c
        {
            return false;
        }
        pos_x += dir_x;
        pos_y += dir_y;
    }

    true
}
