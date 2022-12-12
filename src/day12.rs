use std::collections::{VecDeque, HashSet};
use itertools::Itertools;

struct Map {
    map: Vec<Vec<char>>,
    start_positions: Vec<(usize,usize)>,
    goal: (usize, usize),
}

fn parse(str: &str, pt2: bool) -> Map {
    let mut map = str
        .lines().map(|l| l.trim().chars().collect_vec())
        .collect_vec();
    let start_positions = map.iter().enumerate()
        .filter(|(_, row)| row.contains(&'S') || (pt2 && row.contains(&'a')))
        .map(|(i, row)| (i, row.iter().position(|&c| c == 'S' || (pt2 && c == 'a')).unwrap()))
        .collect_vec();
    for s in &start_positions {
        map[s.0][s.1] = 'a';
    }
    let goal = *map.iter().enumerate()
        .filter(|(_, row)| row.contains(&'E'))
        .map(|(i, row)| (i, row.iter().position(|&c| c == 'E').unwrap()))
        .collect_vec().first().unwrap();
    map[goal.0][goal.1] = 'z';
    Map { map, start_positions, goal }
}

fn neighbours(map: &[Vec<char>], pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let directions = [(1, 0), (-1, 0), (0, -1), (0, 1)];
    for d in directions {
        let (dx, dy) = (pos.0 as i32 + d.0, pos.1 as i32 + d.1);
        if dx < 0 || dx >= map.len() as i32 || dy < 0 || dy >= map[0].len() as i32 {
            continue;
        }
        if map[dx as usize][dy as usize] as i32 - map[pos.0][pos.1] as i32 <= 1 {
            res.push((dx as usize, dy as usize));
        }
    }
    res
}

fn bfs(map: &Map) -> Option<usize> {
    let mut queue = VecDeque::new();
    for start_position in map.start_positions.clone() {
        queue.push_back((start_position, 0));
    }
    let mut explored = HashSet::new();
    while let Some((cs, cost)) = queue.pop_front() {
        if explored.contains(&cs) {
            continue;
        }
        if cs == map.goal {
            return Some(cost);
        }
        for n in neighbours(&map.map, &cs) {
            queue.push_back((n, cost + 1));
        }
        explored.insert(cs);
    }
    None
}

pub fn solve(str: &str) -> (usize, usize) {
    let s1 = bfs(&parse(str, false)).unwrap();
    let s2 = bfs(&parse(str, true)).unwrap();
    (s1, s2)
}