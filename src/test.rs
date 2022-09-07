#![cfg(test)]
use super::*;

#[test]
fn test_fmt() {
    let test_surd = Surd::new(1, 20);
    assert_eq!(format!("{}", test_surd), "2√5");
}

#[test]
fn test_fmt_0() {
    let test_surd = Surd::new(0, 333511);
    assert_eq!(format!("{}", test_surd), "0");
}

#[test]
fn test_fmt_1() {
    let test_surd = Surd::new(1, 333511);
    assert_eq!(format!("{}", test_surd), "√333511");
}

#[test]
fn test_fmt_sqrt1() {
    let test_surd = Surd::new(1, 1);
    assert_eq!(format!("{}", test_surd), "1");
}

#[test]
fn test_massive_square_factors() {
    let test_surd = Surd::new(
        1,
        (2..5).fold(1_usize, |previous, next: usize| previous * next.pow(2)) as u64,
    );
    assert_eq!(format!("{}", test_surd), "331776");
}
