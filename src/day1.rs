use itertools::Itertools;

pub fn solve(str: &str) -> (u64, u64) {
    let sums: Vec<u64> = str.split("\r\n\r\n")
        .map(|e| e.split("\r\n").map(|x| x.parse().unwrap_or(0)))
        .map(|e| e.sum::<u64>())
        .sorted().rev().collect();
    (sums[0], sums.iter().take(3).sum())
}