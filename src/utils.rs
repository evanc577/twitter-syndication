use std::collections::VecDeque;
use std::f64::consts::PI;

pub fn calc_token(id: u64) -> String {
    let x = (id as f64 / 10u64.pow(15) as f64) * PI;

    const BASE: usize = 36;
    static DIGITS: &[u8; BASE] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut result = VecDeque::new();

    // Whole part
    let mut i = x as usize;
    while i > 0 {
        let digit = DIGITS[i % BASE];
        result.push_front(digit);
        i /= BASE;
    }

    // Fractional part
    let mut f = x.fract();
    for _ in 0..8 {
        let digit = DIGITS[(f * BASE as f64) as usize];
        result.push_back(digit);
        f = (f * BASE as f64).fract();
    }

    // Postprocess
    let result: String = result
        .into_iter()
        .filter(|&c| c != b'0')
        .map(|c| c as char)
        .collect();
    if result.is_empty() {
        "0".into()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token() {
        assert_eq!(calc_token(1079631553641164802), "2m7rg9vs6bh");
        assert_eq!(calc_token(1753365478318416281), "49cyp3d8ng");
        assert_eq!(calc_token(1727250580131750107), "46qbfs66quq");
        assert_eq!(calc_token(1757965341978956071), "49et726opy");
    }
}
