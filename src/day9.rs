use itertools::Itertools;
use std::collections::HashSet;

fn neighbours(h: &(i32, i32), t: &(i32, i32)) -> bool {
    let dy = h.0.abs_diff(t.0);
    let dx = h.1.abs_diff(t.1);
    dy <= 1 && dx <= 1
}

fn eval_move(rope: &mut Vec<(i32, i32)>, tail_visited: &mut HashSet<(i32, i32)>, mov: &(&str, i32)) {
    for _ in 0..mov.1 {
        match mov.0 {
            "U" => rope[0] = (rope[0].0 - 1, rope[0].1),
            "D" => rope[0] = (rope[0].0 + 1, rope[0].1),
            "R" => rope[0] = (rope[0].0, rope[0].1 + 1),
            "L" => rope[0] = (rope[0].0, rope[0].1 - 1),
            _ => unreachable!(),
        }
        let tail_idx = rope.len() - 1;
        for j in 1..rope.len() {
            let (p, n) = (rope[j - 1], &mut rope[j]);
            if neighbours(&p, n) {
                break;
            }
            (n.0, n.1) = (n.0 + (p.0 - n.0).signum(), n.1 + (p.1 - n.1).signum());
            if j == tail_idx {
                tail_visited.insert(*n);
            }
        }
    }
}

fn solve_parsed(inp: Vec<(&str, i32)>, rope_c: usize) -> usize {
    let mut tail_visited = HashSet::<(i32, i32)>::new();
    tail_visited.insert((0, 0));
    let mut rope = vec![(0, 0); rope_c];
    inp.iter().for_each(|m| {
        eval_move(&mut rope, &mut tail_visited, m);
    });
    tail_visited.len()
}

pub fn solve(str: &str) -> (usize, usize) {
    let parsed = str
        .lines()
        .map(|l| l.split_once(' ').expect("malformed line"))
        .map(|(d, c)| (d, c.parse::<i32>().expect("well f")))
        .collect_vec();
    (solve_parsed(parsed.clone(), 2), solve_parsed(parsed, 10))
}