use itertools::Itertools;

fn start_marker(s: &str, window_size: usize) -> Option<usize> {
    let chars = s.chars().collect_vec();
    let result = chars
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| {
            window.iter().unique().count() == window_size
        });
    match result {
        Some((i, _)) => Some(i + window_size),
        None => None,
    }
}

pub fn solve(str: &str) -> (usize, usize) {
    (start_marker(str, 4).unwrap(), start_marker(str, 14).unwrap())
}