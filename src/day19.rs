use ahash::AHashSet;
use itertools::Itertools;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Inventory {
    ore: (usize, usize), // rob,c
    clay: (usize, usize),
    obsidian: (usize, usize),
    geode: (usize, usize),
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Inventory {
    fn should_buy(&self, robot: &Robot, blueprint: &BluePrint, max_ore_cost: usize) -> bool {
        match robot {
            Robot::Ore => self.ore.1 >= blueprint.ore_cost && self.ore.0 < max_ore_cost,
            Robot::Clay => self.ore.1 >= blueprint.clay_cost && self.clay.0 < blueprint.obsidian_cost.1,
            Robot::Obsidian => self.ore.1 >= blueprint.obsidian_cost.0 && self.clay.1 >= blueprint.obsidian_cost.1 && self.obsidian.0 < blueprint.geode_cost.1,
            Robot::Geode => self.ore.1 >= blueprint.geode_cost.0 && self.obsidian.1 >= blueprint.geode_cost.1,
        }
    }

    fn buy_robot(&mut self, robot: &Robot, blueprint: &BluePrint) {
        match robot {
            Robot::Ore => { self.ore.1 -= blueprint.ore_cost; self.ore.0 += 1; }
            Robot::Clay => { self.ore.1 -= blueprint.clay_cost; self.clay.0 += 1; }
            Robot::Obsidian => { self.ore.1 -= blueprint.obsidian_cost.0; self.clay.1 -= blueprint.obsidian_cost.1; self.obsidian.0 += 1; }
            Robot::Geode => { self.ore.1 -= blueprint.geode_cost.0; self.obsidian.1 -= blueprint.geode_cost.1; self.geode.0 += 1 }
        }
    }

    fn accumulate(&mut self) {
        self.ore.1 += self.ore.0;
        self.clay.1 += self.clay.0;
        self.obsidian.1 += self.obsidian.0;
        self.geode.1 += self.geode.0;
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct BluePrint {
    id: usize,
    ore_cost: usize,               // ore
    clay_cost: usize,              // ore
    obsidian_cost: (usize, usize), // ore,clay
    geode_cost: (usize, usize),    // ore,obsidian
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct State {
    minutes: usize,
    inventory: Inventory,
    bought_robot: bool,
}

impl State {
    fn heuristic(&self) -> usize {
        let mut h = self.inventory.geode.0 + self.inventory.obsidian.0 + self.inventory.clay.0 + self.inventory.ore.0; 
        if self.bought_robot {
            h += 1;
        }
        h
    }

    fn successors(&self, blueprint: &BluePrint, max_ore_cost: usize) -> Vec<State> {
        let mut succs = Vec::new();
        if self.minutes == 0 {
            succs
        } else {
            for robot in vec![Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode].iter().rev() {
                if self.inventory.should_buy(robot, blueprint, max_ore_cost) {
                    let mut inventory = self.inventory.clone();
                    inventory.accumulate();
                    inventory.buy_robot(robot, blueprint);
                    succs.push(State { minutes: self.minutes - 1, inventory, bought_robot: true });
                }
            }
            let mut inventory = self.inventory.clone();
            inventory.accumulate();
            succs.push(State {
                minutes: self.minutes - 1,
                inventory,
                bought_robot: false
            });
            succs
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.heuristic()).cmp(&(other.heuristic()))
    }
}

fn quality_score(blueprint: &BluePrint, minutes_lim: usize, vote_count: Option<usize>, pt1: bool) -> usize {
    let max_ore_cost = *vec![blueprint.ore_cost, blueprint.clay_cost, blueprint.obsidian_cost.0, blueprint.geode_cost.0].iter().max().unwrap();
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        minutes: minutes_lim,
        inventory: Inventory { ore: (1, 0), clay: (0, 0), obsidian: (0, 0), geode: (0, 0) },
        bought_robot: false
    });
    let mut explored = AHashSet::new();
    let mut max_yield = 0;
    let mut votes = 0;
    while let Some(cur_state) = frontier.pop() {
        if explored.contains(&cur_state) {
            continue;
        }
        if let Some(vc) = vote_count {
            if vc == votes {
                break;
            }
        }
        if cur_state.inventory.geode.1 > 0 {
            if cur_state.inventory.geode.1 > max_yield {
                max_yield = cur_state.inventory.geode.1;
                votes = 0;
            } else {
                votes += 1;
            }
        }
        for suc_state in cur_state.successors(blueprint, max_ore_cost) {
            frontier.push(suc_state);
        }
        explored.insert(cur_state);
    }
    match pt1 {
        true => max_yield * blueprint.id,
        false => max_yield
    }
}

fn parse(str: &str) -> Vec<BluePrint> {
    str.trim()
        .lines()
        .map(|l| l.split(' ').enumerate().collect_vec())
        .map(|x| BluePrint {
            id: x[1].1.trim_matches(':').parse().unwrap(),
            ore_cost: x[6].1.parse().unwrap(),
            clay_cost: x[12].1.parse().unwrap(),
            obsidian_cost: (x[18].1.parse().unwrap(), x[21].1.parse().unwrap()),
            geode_cost: (x[27].1.parse().unwrap(), x[30].1.parse().unwrap()),
        })
        .collect_vec()
}

pub fn solve(str: &str) -> (usize, usize) {
    let parsed = parse(str);
    let s1 = parsed.iter()
        .map(|bp| quality_score(bp, 24, Some(3), true))
        .sum();
    let s2 = parsed.iter()
        .take(3)
        .map(|bp| quality_score(bp, 32, Some(3), false))
        .reduce(|x, y| x * y).unwrap();
    (s1, s2)
}
