use std::collections::{BinaryHeap, HashSet};
use itertools::Itertools;

type Loc = (isize, isize);
#[derive(PartialEq, Eq, Debug)]
struct ExpeditionState {
    g: usize,
    h: usize,
    loc: Loc,
}

impl Ord for ExpeditionState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.g + other.h).cmp(&(self.g + self.h))
    }
}

impl PartialOrd for ExpeditionState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ExpeditionState {
    fn successors(&self, dimensions: (isize, isize), goal: Loc, blizzards: &HashSet<Loc>) -> Vec<ExpeditionState> {
        [(self.loc.0-1,self.loc.1),(self.loc.0+1,self.loc.1),(self.loc.0,self.loc.1-1),(self.loc.0,self.loc.1+1),self.loc].into_iter()
        .filter(move |loc| loc.0 >= 0 && loc.0 < dimensions.0 && loc.1 >= 0 && loc.1 < dimensions.1 && !blizzards.contains(loc) || *loc == (-1,0) || *loc == (dimensions.0, dimensions.1-1))
        .map(move |loc| ExpeditionState {
            g: self.g + 1,
            h: manhatten_dist(loc.clone(), goal),
            loc
        }).collect_vec()
    }
}

fn manhatten_dist(from: Loc, to: Loc) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    North, South, West, East,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Blizzard {
    direction: Direction,
    loc: Loc,
}

fn blizzard_radar(blizzards: &HashSet<Blizzard>, dimensions: (isize, isize), after_ticks: isize) -> HashSet<Loc> {
    blizzards.iter()
        .map(|Blizzard { direction, loc: (row, col) }| match direction {
            Direction::North => ((row-after_ticks).rem_euclid(dimensions.0), *col),
            Direction::South => ((row+after_ticks).rem_euclid(dimensions.0), *col),
            Direction::West => (*row, (col-after_ticks).rem_euclid(dimensions.1)),
            Direction::East => (*row, (col+after_ticks).rem_euclid(dimensions.1)),
        }).collect()
}

fn a_star(s0: ExpeditionState, goal: Loc, blizzards: HashSet<Blizzard>, dimensions: (isize, isize)) -> Option<ExpeditionState> {
    let mut frontier = BinaryHeap::new();
    frontier.push(s0);
    let mut explored = HashSet::new();
    while let Some(sc) = frontier.pop() {
        if explored.contains(&(sc.g, sc.loc)) {
            continue;
        }
        if sc.loc == goal {
            return Some(sc);
        }
        for successor in sc.successors(dimensions, goal, &blizzard_radar(&blizzards, dimensions, sc.g as isize + 1)) {
            frontier.push(successor);
        }
        explored.insert((sc.g, sc.loc));
    }
    None
}

fn parse(str: &str) -> ((isize, isize), HashSet<Blizzard>) {
    let lines = str.trim().lines().map(|l| l.chars().collect_vec()).collect_vec();
    let row_dimension = lines.len() as isize - 2; // dont count the walls
    let col_dimension = lines[0].len() as isize - 2;
    let blizzards = lines.iter().skip(1)
        .enumerate()
        .flat_map(|(row, l)| l.iter().skip(1).enumerate().filter_map(move |(col, ch)| {
            let loc = (row as isize, col as isize);
            match ch {
                '^' => Some(Blizzard { direction: Direction::North, loc }),
                'v' => Some(Blizzard { direction: Direction::South, loc }),
                '<' => Some(Blizzard { direction: Direction::West, loc }),
                '>' => Some(Blizzard { direction: Direction::East, loc }),
                _ => None
            }
        }))
        .collect();
    ((row_dimension, col_dimension), blizzards)
}

pub fn solve(str: &str) -> (usize, usize) {
    let (dimensions, blizzards) = parse(str);
    let (start, goal) = ((-1isize,0isize),(dimensions.0,dimensions.1-1));
    let s0 = ExpeditionState {
        g: 0,
        h: manhatten_dist(start, goal),
        loc: start
    };
    let goal_state = a_star(s0, goal, blizzards.clone(), dimensions).unwrap();
    let s1 = goal_state.g;
    let goal_state = a_star(goal_state, start, blizzards.clone(), dimensions).unwrap();
    let goal_state = a_star(goal_state, goal, blizzards, dimensions).unwrap();
    (s1,goal_state.g)
}