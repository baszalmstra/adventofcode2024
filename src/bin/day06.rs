use adventofcode2024::Direction;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("inputs/day06/input.txt").expect("Can't read data file");
    let mut guard_pos = None;
    let mut guard_dir = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => false,
                    '^' => {
                        guard_pos = Some((x, y));
                        guard_dir = Some(Direction::North);
                        true
                    }
                    _ => true,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let initial_pos = guard_pos.expect("No guard found");
    let mut guard_pos = initial_pos;
    let mut guard_dir = guard_dir.expect("No guard found");
    let mut positions_visited = HashSet::new();
    let mut position_dir_visited = HashSet::new();
    loop {
        positions_visited.insert(guard_pos);
        position_dir_visited.insert((guard_pos, guard_dir));
        let direction = guard_dir.into_direction();
        let (x, y) = (
            guard_pos.0 as i32 + direction.0,
            guard_pos.1 as i32 + direction.1,
        );
        if x < 0 || y < 0 || y >= grid.len() as i32 || x >= grid[y as usize].len() as i32 {
            break;
        }

        if grid[y as usize][x as usize] {
            guard_pos = (x as usize, y as usize);
        } else {
            guard_dir = guard_dir.turn_right();
        }
    }

    println!("Solution 1: {}", positions_visited.len());

    let obstacle_count = position_dir_visited
        .into_iter()
        .filter(|(pos, dir)| {
            if pos.0 == initial_pos.0 && pos.1 == initial_pos.1 {
                return false;
            }
            causes_loop(*pos, *dir, grid.clone())
        })
        .count();
    println!("Solution 2: {}", obstacle_count);
}

fn causes_loop(guard_pos: (usize, usize), guard_dir: Direction, grid: Vec<Vec<bool>>) -> bool {
    let dxdy = guard_dir.into_direction();
    let obstacle_position = (guard_pos.0 as i32 + dxdy.0, guard_pos.1 as i32 + dxdy.1);
    if obstacle_position.0 < 0
        || obstacle_position.1 < 0
        || obstacle_position.1 >= grid.len() as i32
        || obstacle_position.0 >= grid[obstacle_position.1 as usize].len() as i32
    {
        return false;
    }

    let mut guard_pos = (guard_pos.0 as i32, guard_pos.1 as i32);
    let mut guard_dir = guard_dir;
    let mut position_dir_visited = HashSet::new();
    loop {
        if !position_dir_visited.insert((guard_pos, guard_dir)) {
            return true;
        }
        let direction = guard_dir.into_direction();
        let (x, y) = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        if x < 0 || y < 0 || y >= grid.len() as i32 || x >= grid[y as usize].len() as i32 {
            return false;
        }

        if grid[y as usize][x as usize] && !(obstacle_position.0 == x && obstacle_position.1 == y) {
            guard_pos = (x, y);
        } else {
            guard_dir = guard_dir.turn_right();
        }
    }
}
