use std::collections::HashSet;

fn solve(input: &str) -> (i32, Option<i32>) {
    let instructions = input
        .split(", ")
        .map(|s| s.trim())
        .map(|s| {
            let dir = match &s[0..1] {
                "L" => -1,
                "R" => 1,
                _ => panic!("Unknown direction {}", s),
            };
            let length:i32 = s[1..].parse().expect("Not an int");
            (dir, length)
        });

    let mut heading = 0;
    let mut east:i32 = 0;
    let mut north:i32 = 0;
    let mut solution_2 = None;

    let mut visited = HashSet::new();

    for (dir, length) in instructions {
        // % is reminder, not modulo. Therefore we need to do a little tweaking
        heading = ((heading + dir) % 4 + 4) % 4;

        for _ in 0..length {
            match heading {
                0 => north += 1,
                1 => east += 1,
                2 => north -= 1,
                3 => east -= 1,
                _ => panic!("No such heading: {}", heading),
            }
            if solution_2.is_none() {
                if visited.contains(&(east, north)) {
                    solution_2 = Some(north.abs() + east.abs());
                }
                visited.insert((east, north));
            }
        }
    }

    (north.abs() + east.abs(), solution_2)
}

fn solve_1(input: &str) -> i32 {
    solve(input).0
}

fn solve_2(input: &str) -> i32 {
    solve(input).1.unwrap()
}

fn main() {
    assert_eq!(solve_1("R2, L3"), 5);
    assert_eq!(solve_1("R2, R2, R2"), 2);
    assert_eq!(solve_1("R5, L5, R5, R3"), 12);
    assert_eq!(solve_1("R5, R5, R5, R5"), 0);
    assert_eq!(solve_1("R5, R5, R5, R5, R5, R5, R5, R5"), 0);
    assert_eq!(solve_1("L10, R5"), 15);

    assert_eq!(solve_2("R8, R4, R4, R8"), 4);

    let input = include_str!("../../input/01.txt");

    assert_eq!(solve(input), (242, Some(150)));
}