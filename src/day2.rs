pub fn run() {
    let input = include_str!("../data/day2.txt")
        .trim()
        .split_terminator('\n')
        .map(|input| {
            let mut input = input.splitn(2, '-');
            let min = input.next().unwrap().parse::<usize>().unwrap();

            let mut input = input.next().unwrap().splitn(2, ' ');
            let max = input.next().unwrap().parse::<usize>().unwrap();

            let mut input = input.next().unwrap().chars();
            let letter = input.next().unwrap();

            let password: Vec<char> = input.skip(2).collect();

            (min, max, letter, password)
        })
        .collect::<Vec<_>>();

    let a = input
        .iter()
        .filter(|(min, max, letter, password)| {
            let count = password.iter().filter(|c| *c == letter).count();
            count >= *min && count <= *max
        })
        .count();

    let b = input
        .iter()
        .filter(|(i, j, letter, password)| {
            let a = password.get(*i - 1);
            let b = password.get(*j - 1);

            a != b && (a == Some(letter) || b == Some(letter))
        })
        .count();

    println!("day 2: {}, {}", a, b);
}
