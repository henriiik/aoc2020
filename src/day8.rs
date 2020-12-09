use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../data/day8.txt");
    let loop_detected_at = check_input(input);
    let corrected_acc = check_input2(input).unwrap();

    println!("day 8: {} {}", loop_detected_at, corrected_acc);
}

#[derive(Debug, PartialEq)]
enum Op {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

#[derive(Debug)]
struct Machine {
    acc: isize,
    pc: isize,
    reverse: bool,
    ops: Vec<Op>,
    visited: HashSet<isize>,
    tried: HashSet<isize>,
}

impl Machine {
    fn new(ops: Vec<Op>) -> Self {
        Self {
            acc: 0,
            pc: 0,
            reverse: false,
            ops,
            visited: HashSet::new(),
            tried: HashSet::new(),
        }
    }

    fn run_until_loop_detected(&mut self) -> isize {
        loop {
            if !self.visited.insert(self.pc) {
                return self.acc;
            }
            self.exec_next();
        }
    }

    fn exec_next(&mut self) {
        let op = self.ops.get(self.pc as usize).unwrap();
        // println!("pc: {}, op: {:?}", self.pc, &op);
        match op {
            Op::Nop(_) => {
                self.pc += 1;
            }
            Op::Acc(n) => {
                self.acc += n;
                self.pc += 1;
            }
            Op::Jmp(n) => self.pc += n,
        }
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            let mut row = row.split(' ');
            let op = row.next().unwrap();
            let n = row.next().unwrap().parse().unwrap();
            match op {
                "acc" => Op::Acc(n),
                "jmp" => Op::Jmp(n),
                "nop" => Op::Nop(n),
                op => panic!("invalid op: {:?}", op),
            }
        })
        .collect::<Vec<_>>()
}

fn check_input(input: &str) -> isize {
    let mut m = Machine::new(parse_input(input));
    m.run_until_loop_detected()
}

fn check_input2(input: &str) -> Option<isize> {
    run_and_correct_recursive(&parse_input(input), &mut HashSet::new(), 0, 0, false)
}

fn run_and_correct_recursive(
    ops: &[Op],
    visited: &mut HashSet<isize>,
    pc: isize,
    acc: isize,
    has_corrected: bool,
) -> Option<isize> {
    if pc >= ops.len() as isize {
        // we're done!
        return Some(acc);
    }

    if !visited.insert(pc) {
        // loop detected, this op has been visited
        return None;
    }

    let op = ops.get(pc as usize).unwrap();

    let (try_pc, try_acc) = match op {
        Op::Nop(_) => (pc + 1, acc),
        Op::Acc(n) => (pc + 1, acc + n),
        Op::Jmp(n) => (pc + n, acc),
    };

    if let Some(n) = run_and_correct_recursive(ops, visited, try_pc, try_acc, has_corrected) {
        return Some(n);
    }

    if has_corrected {
        visited.remove(&pc);
        return None;
    }

    let try_pc = match op {
        Op::Nop(n) => pc + n,
        Op::Acc(_) => {
            visited.remove(&pc);
            return None;
        }
        Op::Jmp(_) => pc + 1,
    };

    if let Some(n) = run_and_correct_recursive(ops, visited, try_pc, acc, true) {
        return Some(n);
    }

    visited.remove(&pc);
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("../data/day8_test.txt");
        let parsed = parse_input(input);

        assert_eq!(
            parsed,
            vec![
                Op::Nop(0),
                Op::Acc(1),
                Op::Jmp(4),
                Op::Acc(3),
                Op::Jmp(-3),
                Op::Acc(-99),
                Op::Acc(1),
                Op::Jmp(-4),
                Op::Acc(6),
            ]
        );
    }

    #[test]
    fn test_check_input() {
        let input = include_str!("../data/day8_test.txt");
        let loop_detected_at = check_input(input);
        dbg!(loop_detected_at);
        assert_eq!(loop_detected_at, 5);
    }

    #[test]
    fn test_check_input2() {
        let input = include_str!("../data/day8_test.txt");
        let acc = check_input2(input);
        assert_eq!(acc, Some(8));
    }
}
