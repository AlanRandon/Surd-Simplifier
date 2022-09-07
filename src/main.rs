#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

mod test;

use memoize::memoize;
use std::fmt::Display;

#[derive(Debug, Clone)]
struct Surd {
    coefficient: u64,
    radicand: u64,
}

impl Surd {
    const fn new(coefficient: u64, radicand: u64) -> Self {
        Self {
            coefficient,
            radicand,
        }
    }

    fn simplify(self) -> Self {
        let coefficient = self.coefficient;
        if coefficient == 0 {
            return Self::new(0, 0);
        };
        let radicand = self.radicand;
        if is_square(radicand) {
            return Self::new(coefficient * radicand.pow(2), 1);
        }
        if let Some(factor) = get_square_factor(radicand) {
            let sqrt = (factor as f64).sqrt() as u64;
            return Self::new(coefficient * sqrt, radicand / factor).simplify();
        }
        self
    }
}

#[memoize]
fn is_square(n: u64) -> bool {
    sqrt(n) % 1.0 == 0.0
}

#[memoize]
fn sqrt(n: u64) -> f64 {
    (n as f64).sqrt()
}

#[memoize]
const fn divisible_by(n: u64, potential: u64) -> bool {
    n % potential == 0
}

#[memoize]
fn get_square_factor(n: u64) -> Option<u64> {
    let sqrt = sqrt(n).ceil() as u64;
    for i in 2..sqrt {
        if divisible_by(n, i) {
            if is_square(i) {
                return Some(i);
            }
            let i_pair = n / i;
            if is_square(i_pair) {
                return Some(i_pair);
            }
        }
    }
    None
}

impl Display for Surd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            ref coefficient,
            ref radicand,
        } = self.clone().simplify();
        match (coefficient, radicand) {
            (0, _) => write!(f, "0"),
            (_, 1) => write!(f, "{}", coefficient),
            (1, _) => write!(f, "√{}", radicand),
            _ => write!(f, "{}√{}", coefficient, radicand),
        }
    }
}

fn main() {
    println!(
        "{}",
        Surd::new(
            1,
            (2..5).fold(1_usize, |previous, next: usize| previous * next.pow(2)) as u64
        )
    );
}
