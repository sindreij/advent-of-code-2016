fn solve_1(input: &str) -> Vec<i32> {
    let lines:Vec<_> = input.split_terminator('\n').collect();

    let mut x = 1;
    let mut y = 1;
    let mut result = Vec::new();

    for line in lines {
        for op in line.chars() {
            match op {
                'U' => y -= 1,
                'D' => y += 1,
                'R' => x += 1,
                'L' => x -= 1,
                _ => panic!("No such operator, {}", op),
            }
            x = x.min(2);
            x = x.max(0);
            y = y.min(2);
            y = y.max(0);
        }

        result.push(1 + x + 3 * y);
    }

    result
}

fn next(pos: (i32, i32), mov: (i32, i32)) -> (i32, i32) {
    let new_pos = (pos.0 + mov.0, pos.1 + mov.1);
    if new_pos.0.abs() * new_pos.1.abs() > 1 {
        return pos;
    }
    if new_pos.0.abs() > 2 || new_pos.1.abs() > 2 {
        return pos;
    }

    new_pos
}

static CHARS: [char; 23] = ['X', 'X', '1', 'X', 'X', 'X', '2', '3', '4', 'X', '5', '6', '7', '8', '9', 'X', 'A', 'B', 'C', 'X', 'X', 'X', 'D'];

fn solve_2(input: &str) -> Vec<char> {
    let lines:Vec<_> = input.split_terminator('\n').collect();

    let mut x = -2;
    let mut y = 0;
    let mut result = Vec::new();

    for line in lines {
        for op in line.chars() {
            let (x_, y_) = match op {
                'U' => next((x, y), (0, -1)),
                'D' => next((x, y), (0, 1)),
                'R' => next((x, y), (1, 0)),
                'L' => next((x, y), (-1, 0)),
                _ => panic!("No such operator, {}", op),
            };
            x = x_;
            y = y_;
        }

        result.push(CHARS[((x + 2) + 5 * (y + 2)) as usize]);
    }

    result
}

fn main() {
    assert_eq!(solve_1("ULL\nRRDDD\nLURDL\nUUUUD").as_slice(), &[1,9,8,5]);
    assert_eq!(solve_2("ULL\nRRDDD\nLURDL\nUUUUD").as_slice(), &['5', 'D', 'B', '3']);

    let input = include_str!("../../input/02.txt");
    assert_eq!(solve_1(input).as_slice(), &[8,2,9,5,8]);
    assert_eq!(solve_2(input).as_slice(), &['B', '3', 'D', 'B', '8']);
}