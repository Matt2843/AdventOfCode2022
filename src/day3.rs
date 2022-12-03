use std::collections::HashSet;
use itertools::Itertools;

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

pub fn solve(str: &str) -> (u32, u32) {
    let s1 = str.lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(c1,c2)| &HashSet::<char>::from_iter(c1.chars()) & &HashSet::<char>::from_iter(c2.chars()))
        .map(|x| x.iter().fold(0, |ac, y| ac + priority(*y)))
        .sum();
    let s2 = str.lines()
        .chunks(3)
        .into_iter()
        .map(|x| x
            .map(|y| HashSet::<char>::from_iter(y.chars())) // and reduce to a single set..
            .reduce(|x, y| &x & &y)
            .unwrap()
        )
        .map(|x| x.iter().fold(0, |acc, y| acc + priority(*y)))
        .sum();
    (s1, s2)
}