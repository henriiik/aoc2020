use eyre::{bail, Result};
use std::{fmt::Debug, iter::Peekable};
use tracing::{debug, info, instrument};

#[instrument]
pub fn run() -> Result<()> {
    let input = include_str!("../data/day18.txt");

    let sum = calculate(input)?;
    info!(?sum);

    let sum_advanced = calculate_advanced(input)?;
    info!(?sum_advanced);

    Ok(())
}

#[derive(Clone, PartialEq)]
enum Expr {
    Simple(u64),
    Complex {
        lhs: Box<Expr>,
        op: Operator,
        rhs: Box<Expr>,
    },
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Simple(n) => write!(f, "{}", n),
            Expr::Complex { lhs, op, rhs } => write!(f, "({:?} {:?} {:?})", lhs, op, rhs),
        }
    }
}

impl Expr {
    fn evaluate(self) -> u64 {
        match self {
            Expr::Simple(n) => n,
            Expr::Complex { lhs, op, rhs } => op.evaluate(*lhs, *rhs),
        }
    }

    #[instrument(skip(iter))]
    fn parse<I: Iterator<Item = Token>>(iter: &mut Peekable<I>) -> Result<Self> {
        let mut expr = match iter.next() {
            Some(t) => match t {
                Token::Operator(_) => bail!("unexpected operator"),
                Token::Num(n) => Expr::Simple(n),
                Token::OpenParen => Expr::parse(iter)?,
                Token::CloseParen => bail!("unexpected close paren"),
            },
            None => bail!("unexpected end of input"),
        };

        loop {
            let op = match iter.next() {
                Some(Token::Operator(op)) => op,
                Some(Token::CloseParen) | None => return Ok(expr),
                Some(t) => bail!("unexpected token: {:?}", t),
            };

            let rhs = match iter.next() {
                Some(t) => match t {
                    Token::Operator(_) => bail!("unexpected operator"),
                    Token::Num(n) => Expr::Simple(n),
                    Token::OpenParen => Expr::parse(iter)?,
                    Token::CloseParen => bail!("unexpected close paren"),
                },
                None => bail!("unexpected end of input"),
            };

            expr = Self::Complex {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            }
        }
    }

    #[instrument(skip(iter))]
    fn parse_advanced<I: Iterator<Item = Token>>(iter: &mut Peekable<I>) -> Result<Self> {
        let mut lhs = match iter.next() {
            Some(t) => match t {
                Token::Operator(_) => bail!("unexpected operator"),
                Token::Num(n) => Expr::Simple(n),
                Token::OpenParen => Expr::parse_advanced(iter)?,
                Token::CloseParen => bail!("unexpected close paren"),
            },
            None => bail!("unexpected end of input"),
        };

        loop {
            let op = match iter.next() {
                Some(Token::Operator(Operator::Add)) => Operator::Add,
                Some(Token::Operator(Operator::Mul)) => {
                    debug!(?lhs);
                    let rhs = Expr::parse_advanced(iter)?;
                    debug!(?rhs);
                    let expr = Self::Complex {
                        lhs: Box::new(lhs),
                        op: Operator::Mul,
                        rhs: Box::new(rhs),
                    };
                    debug!(?expr);
                    return Ok(expr);
                }
                Some(Token::CloseParen) | None => return Ok(lhs),
                Some(t) => bail!("unexpected token: {:?}", t),
            };

            let rhs = match iter.next() {
                Some(t) => match t {
                    Token::Operator(_) => bail!("unexpected operator"),
                    Token::Num(n) => Expr::Simple(n),
                    Token::OpenParen => Expr::parse_advanced(iter)?,
                    Token::CloseParen => bail!("unexpected close paren"),
                },
                None => bail!("unexpected end of input"),
            };

            lhs = Self::Complex {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Mul,
}

impl Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Mul => write!(f, "*"),
        }
    }
}

impl Operator {
    fn evaluate(&self, lhs: Expr, rhs: Expr) -> u64 {
        match self {
            Operator::Add => lhs.evaluate() + rhs.evaluate(),
            Operator::Mul => lhs.evaluate() * rhs.evaluate(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Operator(Operator),
    Num(u64),
    OpenParen,
    CloseParen,
}

fn tokenize(input: &str) -> Vec<Vec<Token>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| match c {
                    ' ' => None,
                    '(' => Some(Token::OpenParen),
                    ')' => Some(Token::CloseParen),
                    '+' => Some(Token::Operator(Operator::Add)),
                    '*' => Some(Token::Operator(Operator::Mul)),
                    _ => {
                        let n = c.to_digit(10).unwrap() as u64;
                        if n < 1 || n > 10 {
                            panic!("invalid n ! : {}", n);
                        }
                        Some(Token::Num(n))
                    }
                })
                .collect()
        })
        .collect()
}

fn calculate(input: &str) -> Result<u64> {
    let exprs = tokenize(input)
        .into_iter()
        .map(|tokens| Ok(Expr::parse(&mut tokens.into_iter().peekable())?))
        .collect::<Result<Vec<_>>>()?;

    Ok(exprs
        .into_iter()
        .zip(input.lines())
        .map(|(expr, line)| {
            debug!(?line);
            debug!(?expr);
            expr.evaluate()
        })
        .sum())
}

fn calculate_advanced(input: &str) -> Result<u64> {
    let exprs = tokenize(input)
        .into_iter()
        .map(|tokens| Ok(Expr::parse_advanced(&mut tokens.into_iter().peekable())?))
        .collect::<Result<Vec<_>>>()?;

    Ok(exprs
        .into_iter()
        .zip(input.lines())
        .map(|(expr, line)| {
            debug!(?line);
            debug!(?expr);
            expr.evaluate()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("../data/day18_test.txt");

        let exprs = tokenize(input)
            .into_iter()
            .map(|tokens| Expr::parse(&mut tokens.into_iter().peekable()))
            .collect::<Result<Vec<_>>>()?;

        let want = vec![71, 51, 26, 437, 12240, 13632];
        let want_sum: u64 = want.iter().sum();

        for ((expr, want), input) in exprs.into_iter().zip(want.into_iter()).zip(input.lines()) {
            debug!(?input);
            debug!(?expr);
            let got = expr.evaluate();
            assert_eq!(got, want);
        }

        let got_sum = calculate(input)?;
        assert_eq!(want_sum, got_sum);

        Ok(())
    }

    #[test]
    fn test_parse_advanced() -> Result<()> {
        let input = include_str!("../data/day18_test.txt");

        let exprs = tokenize(input)
            .into_iter()
            .map(|tokens| Expr::parse_advanced(&mut tokens.into_iter().peekable()))
            .collect::<Result<Vec<_>>>()?;

        let want = vec![231, 51, 46, 1445, 669060, 23340];
        let want_sum: u64 = want.iter().sum();

        for ((expr, want), input) in exprs.into_iter().zip(want.into_iter()).zip(input.lines()) {
            debug!(?input);
            debug!(?expr);
            let got = expr.evaluate();
            assert_eq!(got, want);
        }

        let got_sum = calculate_advanced(input)?;
        assert_eq!(want_sum, got_sum);

        Ok(())
    }

    #[test]
    fn test_parse_2() -> Result<()> {
        let tests = vec![
            ("1 + 2", 3),
            ("2 * (2 + 2)", 8),
            ("2 * 2 + 2", 6),
            ("(2 * 2) + 2", 6),
            ("2 * 2 + 2 * 2", 12),
            ("2 * 2 + (2 * 2)", 8),
            ("(2 * 2) + 2 * 2", 12),
            ("((2 * 2) + 2) * 2", 12),
            ("(((2 * 2) + 2) * 2)", 12),
        ];

        for (input, want) in tests {
            debug!(?input, want);
            let tokens = tokenize(input).pop().unwrap();
            let expr = Expr::parse(&mut tokens.into_iter().peekable()).unwrap();
            let got = expr.evaluate();
            assert_eq!(got, want);
        }

        Ok(())
    }
}
