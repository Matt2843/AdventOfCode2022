use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;

const DEBUG: bool = false;

#[derive(Debug)]
struct State {
    g: usize,
    h: usize,
    pos: (usize, usize),
    s: char,
    parent: Option<Rc<State>>,
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

fn neighbours(state: Rc<State>, arr: &[Vec<char>], end: (usize, usize)) -> Vec<Rc<State>> {
    let mut res = Vec::new();
    let directions = [(1, 0), (-1, 0), (0, -1), (0, 1)];
    for d in directions {
        let (dx, dy) = (state.pos.0 as i32 + d.0, state.pos.1 as i32 + d.1);
        if dx < 0 || dx >= arr.len() as i32 || dy < 0 || dy >= arr[0].len() as i32 {
            continue;
        }
        if arr[dx as usize][dy as usize] as i32 - state.s as i32 <= 1 {
            res.push(Rc::new(State {
                g: state.g + 1,
                pos: (dx as usize, dy as usize),
                s: arr[dx as usize][dy as usize],
                h: manhatten_dist((dx as usize, dy as usize), end),
                parent: Some(state.clone()),
            }));
        }
    }
    res
}

#[derive(Clone)]
struct Map {
    arr: Vec<Vec<char>>,
    starts: Vec<Rc<State>>,
    goal: (usize, usize),
}

fn find_char_indices(arr: &[Vec<char>], chr: char) -> Vec<(usize, usize)> {
    arr.iter()
        .enumerate()
        .filter(|(_, row)| row.contains(&chr))
        .map(|(i, row)| (i, row.iter().position(|&c| c == chr).unwrap()))
        .collect()
}

fn parse(str: &str, extend_as: bool) -> Map {
    let mut arr = str
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();
    let start = *find_char_indices(&arr, 'S').first().unwrap();
    let goal = *find_char_indices(&arr, 'E').first().unwrap();
    arr[start.0][start.1] = 'a';
    arr[goal.0][goal.1] = 'z';
    let mut starts = vec![start];
    if extend_as {
        starts.extend(find_char_indices(&arr, 'a'));
    }
    Map {
        arr,
        starts: starts
            .iter()
            .map(|s| Rc::new(State {
                g: 0,
                h: manhatten_dist(*s, goal),
                pos: *s,
                s: 'a',
                parent: None,
            }))
            .collect_vec(),
        goal,
    }
}

fn print_map(map: &Map, path: Option<&Vec<(usize, usize)>>) {
    let mut map_clone = map.clone();
    if let Some(p) = path {
        for pos in p {
            map_clone.arr[pos.0][pos.1] = ' ';
        }
    }
    for x in map_clone.arr.iter() {
        for y in x.iter() {
            print!("{}", y);
        }
        println!();
    }
}

fn retrace_path(state: Rc<State>) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    path.push(state.pos);
    let mut cur_state = state;
    while let Some(parent) = cur_state.parent.clone() {
        cur_state = parent;
        path.push(cur_state.pos);
    }
    path
}


fn a_star(map: &Map) -> Option<usize> {
    let mut pq: BinaryHeap<Rc<State>> = BinaryHeap::new();
    for x in map.starts.clone() {
        pq.push(x);
    }
    let mut explored = HashSet::new();
    while let Some(current_state) = pq.pop() {
        if explored.contains(&current_state.pos) {
            continue;
        }
        if current_state.pos == map.goal {
            if DEBUG {
                print_map(map, Some(&retrace_path(current_state.clone())));
            }
            return Some(current_state.g);
        }
        for n in neighbours(current_state.clone(), &map.arr, map.goal) {
            pq.push(n);
        }
        explored.insert(current_state.pos);
    }
    None
}

pub fn solve(str: &str) -> (usize, usize) {
    let s1 = a_star(&parse(str, false)).unwrap();
    let s2 = a_star(&parse(str, true)).unwrap();
    (s1, s2)
}