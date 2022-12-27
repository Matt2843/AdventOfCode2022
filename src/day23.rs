use itertools::Itertools;
use ahash::{AHashMap, AHashSet};
use std::collections::VecDeque;

type Goblin = (i32, i32);
fn parse(str: &str) -> AHashSet<Goblin> {
    str.lines()
        .enumerate()
        .map(|(i, l)| (i, l.chars().enumerate().collect_vec()))
        .fold(AHashSet::new(), |mut acc, (row, y)| {
            for (col, ch) in y {
                if ch == '#' {
                    acc.insert((row as i32, col as i32));
                }
            }
            acc
        })
}

fn move_north(goblin: &Goblin, goblins: &AHashSet<Goblin>) -> Option<Goblin> {
    [(-1,-1),(-1,0),(-1,1)].iter()
        .all(|(ro, co)| !goblins.contains(&(goblin.0 + ro, goblin.1 + co)))
        .then_some((goblin.0-1, goblin.1))
}

fn move_south(goblin: &Goblin, goblins: &AHashSet<Goblin>) -> Option<Goblin> {
    [(1,-1),(1,0),(1,1)].iter()
        .all(|(ro, co)| !goblins.contains(&(goblin.0 + ro, goblin.1 + co)))
        .then_some((goblin.0+1, goblin.1))
}

fn move_west(goblin: &Goblin, goblins: &AHashSet<Goblin>) -> Option<Goblin> {
    [(-1,-1),(0,-1),(1,-1)].iter()
        .all(|(ro, co)| !goblins.contains(&(goblin.0 + ro, goblin.1 + co)))
        .then_some((goblin.0, goblin.1-1))
}

fn move_east(goblin: &Goblin, goblins: &AHashSet<Goblin>) -> Option<Goblin> {
    [(-1,1),(0,1),(1,1)].iter()
        .all(|(ro, co)| !goblins.contains(&(goblin.0 + ro, goblin.1 + co)))
        .then_some((goblin.0, goblin.1+1))
}

fn has_adjacent_goblins(goblin: &Goblin, goblins: &AHashSet<Goblin>) -> bool {
    [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)].iter()
        .any(|(ro,co)| goblins.contains(&(goblin.0 + ro, goblin.1 + co)))
}

fn get_rect(goblins: &AHashSet<Goblin>) -> (Goblin, Goblin) {
    let (row_bounds, col_bounds) = goblins.iter()
        .fold(((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)), |((row_min, row_max), (col_min, col_max)), gob| {
            ((row_min.min(gob.0), row_max.max(gob.0)), (col_min.min(gob.1), col_max.max(gob.1)))
        });
    (row_bounds, col_bounds)
}

fn empty_spaces(goblins: &AHashSet<Goblin>) -> usize {
    let ((row_min, row_max),(col_min, col_max)) = get_rect(goblins);
    ((row_max+1 - row_min) * (col_max+1 - col_min) - goblins.len() as i32) as usize
}

fn move_gobs(goblins: &mut AHashSet<Goblin>, iterations: usize) -> usize {
    let mut moves = VecDeque::from([move_north, move_south, move_west, move_east]);
    let mut d_moves = VecDeque::from(["NORTH", "SOUTH", "WEST", "EAST"]);
    let mut kappa = 0;
    for _ in 0..iterations {
        kappa += 1;
        let mut proposed_moves = AHashMap::<Goblin, Vec<Goblin>>::new();
        for gob in goblins.iter().filter(|g| has_adjacent_goblins(g, goblins)) {
            if let Some(m_gob) = moves.iter().flat_map(|m| m(gob, &goblins)).next() {
                proposed_moves.entry(m_gob).and_modify(|gobs| gobs.push(*gob)).or_insert(vec![*gob]);
            }
        }
        if proposed_moves.is_empty() {
            return kappa;
        }
        for (m_gob, o_gob) in proposed_moves.iter().filter(|(_, gobs)| gobs.len() == 1) {
            goblins.remove(o_gob.first().unwrap());
            goblins.insert(*m_gob);
        }
        moves.rotate_left(1);
        d_moves.rotate_left(1);
    }
    empty_spaces(goblins)
}

pub fn solve(str: &str) -> (usize, usize) {
    let mut goblins = parse(str);
    let s1 = move_gobs(&mut goblins.clone(), 10);
    let s2 = move_gobs(&mut goblins, usize::MAX);
    (s1, s2)
}

















