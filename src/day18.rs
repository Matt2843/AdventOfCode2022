use ahash::AHashSet;
use std::collections::VecDeque;
use itertools::Itertools;

fn bfs_trapped_in_n(input: &AHashSet<(i64, i64, i64)>, (x,y,z): (i64, i64, i64), n: usize) -> bool {
    let mut queue = VecDeque::new();
    let mut explored = AHashSet::new();
    queue.push_back((x, y, z));
    while let Some((x, y, z)) = queue.pop_front() {
        if explored.contains(&(x,y,z)) || input.contains(&(x,y,z)) {
            continue;
        }
        if explored.len() > n {
            return false;
        }
        queue.push_back((x+1,y,z));
        queue.push_back((x-1,y,z));
        queue.push_back((x,y+1,z));
        queue.push_back((x,y-1,z));
        queue.push_back((x,y,z+1));
        queue.push_back((x,y,z-1));
        explored.insert((x,y,z));
    }
    true
}

const DIRECTIONS: [(i64,i64,i64);6] = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];
fn surface_area(input: &AHashSet<(i64, i64, i64)>) -> usize {
    input.iter().cloned()
        .flat_map(|(x, y, z)| DIRECTIONS.iter().map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz)))
        .filter(|cube| !input.contains(cube))
        .count()
}
fn surface_area_edge(input: &AHashSet<(i64, i64, i64)>, n: usize) -> usize {
    input.iter().cloned()
        .flat_map(|(x, y, z)| DIRECTIONS.iter().map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz)))
        .filter(|cube| !bfs_trapped_in_n(input, *cube, n))
        .count()
}

pub fn solve(str: &str) -> (usize, usize) {
    let parsed = str.lines()
        .map(|l| l.splitn(3,',').map(|s| s.parse::<i64>().unwrap()).collect_tuple::<(i64,i64,i64)>().unwrap())
        .fold(AHashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        });
    (surface_area(&parsed), surface_area_edge(&parsed, 2000))
}