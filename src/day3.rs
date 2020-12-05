pub fn run() {
    let input = include_str!("day3.txt")
        .trim()
        .split_terminator('\n')
        .collect::<Vec<_>>();

    let a: (usize, usize) = input.iter().skip(1).fold((0, 0), |(mut n, x), input| {
        let x = (x + 3) % input.len();

        let tree = input.get(x..=x).expect("should not go out of bounds");
        if tree == "#" {
            n += 1;
        }

        (n, x)
    });

    let b = calc(&input, 1, 1);
    let c = calc(&input, 3, 1);
    let d = calc(&input, 5, 1);
    let e = calc(&input, 7, 1);
    let f = calc(&input, 1, 2);

    println!("day 3: {}, {}", a.0, b * c * d * e * f);
}

fn calc(input: &[&str], right: usize, down: usize) -> usize {
    input
        .iter()
        .fold((0, 0, 0), |(mut n, mut x, mut y), input| {
            if y == down {
                x = (x + right) % input.len();
                let tree = input.get(x..=x).expect("should not go out of bounds");
                if tree == "#" {
                    n += 1;
                }
                y = 1;
            } else {
                y += 1;
            }
            // dbg!((n, x, y, input));
            (n, x, y)
        })
        .0
}
