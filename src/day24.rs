use itertools::Itertools;
use std::hash::Hash;
use ahash::{AHashMap, AHashSet};
use std::collections::BinaryHeap;

type Position = (i32, i32);
#[derive(PartialEq, Eq, Debug, Clone)]
struct BlizzardState {
    g: usize,
    h: usize,
    location: Position,
    map: AHashMap<Position, Vec<MapEntity>>
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum MapEntity {
    NWind,
    SWind,
    WWind,
    EWind,
    Wall,
    Empty,
}

impl PartialOrd for BlizzardState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BlizzardState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.g + other.h).cmp(&(self.g + self.h))
        //other.g.cmp(&self.g)
    }
}

impl BlizzardState {
    fn get_rect(&self) -> (Position, Position) {
        let (row_bounds, col_bounds) = self.map.keys().fold(
            ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
            |((row_min, row_max), (col_min, col_max)), gob| {
                (
                    (row_min.min(gob.0), row_max.max(gob.0)),
                    (col_min.min(gob.1), col_max.max(gob.1)),
                )
            },
        );
        (row_bounds, col_bounds)
    }
    fn print(&self) {
        // we need to find the key-rect...
        let ((row_min, row_max), (col_min, col_max)) = self.get_rect();
        let mut c_map = self.map.clone();
        for x in row_min..=row_max {
            for y in col_min..=col_max {
                let entries = c_map.get_mut(&(x,y)).unwrap();
                if self.location == (x, y) {
                    print!("E")
                } else if entries.iter().all(|e| *e == MapEntity::Empty) {
                    print!(".")
                } else {
                    entries.retain(|e| *e != MapEntity::Empty);
                    if entries.len() == 1 {
                        match *entries.first().unwrap() {
                            MapEntity::NWind => print!("^"),
                            MapEntity::SWind => print!("v"),
                            MapEntity::EWind => print!(">"),
                            MapEntity::WWind => print!("<"),
                            MapEntity::Wall => print!("#"),
                            _ => unreachable!()
                        }
                    } else {
                        print!("{:?}", entries.len())
                    }
                }
            }
            println!()
        }
        println!()
    }
    fn blizzard_tick(&self) -> AHashMap<Position, Vec<MapEntity>> {
        let mut next_map = AHashMap::<Position, Vec<MapEntity>>::new();
        let ((min_row, max_row), (min_col, max_col)) = self.get_rect();
        for (pos, entity_vec) in self.map.iter() {
            for entity in entity_vec {
                match *entity {
                    MapEntity::NWind => {
                        let mut next_pos = (pos.0-1, pos.1);
                        if self.map[&next_pos].iter().all(|e| *e == MapEntity::Wall) {
                            next_pos = (max_row-1, pos.1);
                        } 
                        next_map.entry(next_pos)
                            .and_modify(|v| v.push(MapEntity::NWind))
                            .or_insert_with(|| vec![MapEntity::NWind]);
                    }
                    MapEntity::SWind => {
                        let mut next_pos = (pos.0+1, pos.1);
                        if self.map[&next_pos].iter().all(|e| *e == MapEntity::Wall) {
                            next_pos = (min_row+1, pos.1);
                        } 
                        next_map.entry(next_pos)
                            .and_modify(|v| v.push(MapEntity::SWind))
                            .or_insert_with(|| vec![MapEntity::SWind]);
                    }
                    MapEntity::WWind => {
                        let mut next_pos = (pos.0, pos.1-1);
                        if self.map[&next_pos].iter().all(|e| *e == MapEntity::Wall) {
                            next_pos = (pos.0, max_col-1);
                        } 
                        next_map.entry(next_pos)
                            .and_modify(|v| v.push(MapEntity::WWind))
                            .or_insert_with(|| vec![MapEntity::WWind]);
                    }
                    MapEntity::EWind => {
                        let mut next_pos = (pos.0, pos.1+1);
                        if self.map[&next_pos].iter().all(|e| *e == MapEntity::Wall) {
                            next_pos = (pos.0, min_col+1);
                        } 
                        next_map.entry(next_pos)
                            .and_modify(|v| v.push(MapEntity::EWind))
                            .or_insert_with(|| vec![MapEntity::EWind]);
                    }
                    MapEntity::Wall => {next_map.insert(*pos, vec![MapEntity::Wall]);}
                    MapEntity::Empty => {next_map.entry(*pos).and_modify(|v| v.push(MapEntity::Empty)).or_insert_with(|| vec![MapEntity::Empty]);},
                }
                if [MapEntity::EWind, MapEntity::SWind, MapEntity::NWind, MapEntity::WWind].contains(entity) {
                    next_map.entry(*pos).and_modify(|v| v.push(MapEntity::Empty)).or_insert_with(|| vec![MapEntity::Empty]);
                }
            }
        }
        next_map
    }
    fn successors(&self, goal: Position) -> Vec<BlizzardState> {
        let mut successors_states = Vec::new();
        let next_map = self.blizzard_tick();
        for (ro, co) in [(-1,0),(0,-1),(0,1),(1,0)] {
            let next_location = (self.location.0 + ro, self.location.1 + co);
            if !next_map.contains_key(&next_location) {
                continue;
            }
            if next_map[&next_location].iter().all(|e| *e == MapEntity::Empty) {
                successors_states.push(BlizzardState {
                    g: self.g + 1,
                    h: manhatten_dist(next_location, goal),
                    map: next_map.clone(),
                    location: next_location
                })
            }
        }
        if next_map[&self.location].iter().all(|e| *e == MapEntity::Empty) {
            successors_states.push(BlizzardState {
                g: self.g + 1,
                map: next_map,
                ..*self
            });
        }
        successors_states
    }
}

fn manhatten_dist(from: Position, to: Position) -> usize {
    let dx = from.0.abs_diff(to.0);
    let dy = from.1.abs_diff(to.1);
    (dx + dy) as usize
}

fn parse(str: &str) -> (BlizzardState, Position) {
    let map = str.trim().lines()
        .enumerate()
        .map(|(row, l)| (row as i32,l.chars().collect_vec()))
        .fold(AHashMap::<Position, Vec<MapEntity>>::new(), |mut acc, (r, row)| {
            for (c, ch) in row.iter().enumerate() {
                match *ch {
                    '^' => {acc.insert((r,c as i32), vec![MapEntity::NWind]);},
                    'v' => {acc.insert((r,c as i32), vec![MapEntity::SWind]);},
                    '<' => {acc.insert((r,c as i32), vec![MapEntity::WWind]);},
                    '>' => {acc.insert((r,c as i32), vec![MapEntity::EWind]);},
                    '#' => {acc.insert((r,c as i32), vec![MapEntity::Wall]);},
                    '.' => {acc.insert((r,c as i32), vec![MapEntity::Empty]);},
                    _ => unreachable!()
                }
            }
            acc
        });
    let start = *map.iter().find(|(k,v)| k.0 == 0 && v.first().unwrap() == &MapEntity::Empty).unwrap().0;
    let goal = *map.iter().find(|(k, v)| k.0 == map.iter().max_by_key(|(k, _)| k.0).unwrap().0.0 && v.first().unwrap() == &MapEntity::Empty).unwrap().0;
    (BlizzardState {
        g: 0,
        h: manhatten_dist(start, goal),
        location: start,
        map
    }, goal)
}

fn a_star(s_0: BlizzardState, s_g: Position) -> Option<BlizzardState> {
    let mut frontier = BinaryHeap::<BlizzardState>::new();
    frontier.push(s_0);
    let mut explored = AHashSet::new();
    while let Some(s_c) = frontier.pop() {
        //s_c.print();
        if explored.len() % 10_000 == 0 {
            s_c.print();
            println!("explored: {:?}", explored.len())
        }
        if explored.contains(&(s_c.g, s_c.location)) {
            continue;
        }
        if s_c.location == s_g {
            return Some(s_c);
        }
        for succ in s_c.successors(s_g) {
            frontier.push(succ);
        }
        explored.insert((s_c.g, s_c.location));
    }
    None
}

pub fn solve(str: &str) -> (usize, usize) {
    let (s_0, s_g) = parse(str);
    let goal_state = a_star(s_0.clone(), s_g).unwrap();
    let s1 = goal_state.g;
    let goal_state = a_star(goal_state, s_0.location).unwrap();
    let goal_state = a_star(goal_state, s_g).unwrap();
    (s1,goal_state.g)
}