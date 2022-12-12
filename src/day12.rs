use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use itertools::Itertools;

#[derive(Debug)]
struct State {
    g: usize,
    h: usize,
    pos: (usize, usize),
    s: char,
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_f = self.g + self.h;
        let other_f = other.g + other.h;
        other_f.partial_cmp(&self_f)
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        other.pos == self.pos
    }
}
impl Eq for State {}

fn manhatten_dist(from: (usize, usize), to: (usize, usize)) -> usize {
    let dx = from.0.abs_diff(to.0);
    let dy = from.1.abs_diff(to.1);
    dx + dy
}

fn neighbours(state: &State, arr: &[Vec<char>], end: (usize, usize)) -> Vec<State> {
    let mut res = Vec::new();
    let directions = [(1, 0), (-1, 0), (0, -1), (0, 1)];
    for d in directions {
        let (dx, dy) = (state.pos.0 as i32 + d.0, state.pos.1 as i32 + d.1);
        if dx < 0 || dx >= arr.len() as i32 || dy < 0 || dy >= arr[0].len() as i32 {
            continue;
        }
        if arr[dx as usize][dy as usize] as i32 - state.s as i32 <= 1
        {
            res.push(State { g: state.g + 1, pos: (dx as usize, dy as usize),
                s: arr[dx as usize][dy as usize],
                h: manhatten_dist((dx as usize, dy as usize), end),
            });
        }
    }
    res
}

#[derive(Clone)]
struct Map {
    arr: Vec<Vec<char>>,
    start: (usize, usize),
    goal: (usize, usize),
}

fn find_char_indices(arr: &[Vec<char>], chr: char) -> Vec<(usize, usize)> {
    arr.iter()
        .enumerate()
        .filter(|(_, row)| row.contains(&chr))
        .map(|(i, row)| (i, row.iter().position(|&c| c == chr).unwrap()))
        .collect()
}

fn parse(str: &str) -> (Map, Vec<(usize, usize)>) {
    let mut arr = str.lines().map(|l| l.trim().chars().collect_vec()).collect_vec();
    let start = *find_char_indices(&arr, 'S').first().unwrap();
    let goal = *find_char_indices(&arr, 'E').first().unwrap();
    arr[start.0][start.1] = 'a';
    arr[goal.0][goal.1] = 'z';
    let a_vec = find_char_indices(&arr, 'a');
    (Map { arr, start, goal }, a_vec)
}

fn a_star(map: &Map) -> Option<usize> {
    let start = State { g: 0, pos: map.start, 
        s: map.arr[map.start.0][map.start.1],
        h: manhatten_dist(map.start, map.goal),
    };
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    pq.push(start);
    let mut explored = HashSet::new();
    while !pq.is_empty() {
        let current_state = pq.pop().unwrap();
        if current_state.pos == map.goal {
            return Some(current_state.g);
        }
        for n in neighbours(&current_state, &map.arr, map.goal) {
            if !explored.contains(&n.pos) {
                pq.push(n);
            }
        }
        explored.insert(current_state.pos);
    }
    None
}

pub fn solve(str: &str) -> (usize, usize) {
    let (map, a_vec) = parse(str);
    let s1 = a_star(&map).unwrap();
    let s2 = a_vec.iter()
        .map(|s| {
            let mut new_map = map.clone();
            new_map.start = *s;
            new_map
        }).map(|m| a_star(&m).unwrap())
        .min().unwrap();
    (s1, s2)
}