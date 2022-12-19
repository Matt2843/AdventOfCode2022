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

enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Inventory {
    fn can_afford(&self, robot: &Robot, blueprint: &BluePrint) -> bool {
        match robot {
            Robot::Ore => self.ore.1 >= blueprint.ore_cost,
            Robot::Clay => self.ore.1 >= blueprint.clay_cost,
            Robot::Obsidian => {
                self.ore.1 >= blueprint.obsidian_cost.0 && self.clay.1 >= blueprint.obsidian_cost.1
            }
            Robot::Geode => {
                self.ore.1 >= blueprint.geode_cost.0 && self.obsidian.1 >= blueprint.geode_cost.1
            }
        }
    }
    fn buy_robot(&mut self, robot: &Robot, blueprint: &BluePrint) {
        if !self.can_afford(robot, blueprint) {
            panic!("tried to buy robot that we can't afford!?")
        }
        match robot {
            Robot::Ore => {
                self.ore.1 -= blueprint.ore_cost;
                self.ore.0 += 1;
            }
            Robot::Clay => {
                self.ore.1 -= blueprint.clay_cost;
                self.clay.0 += 1;
            }
            Robot::Obsidian => {
                self.ore.1 -= blueprint.obsidian_cost.0;
                self.clay.1 -= blueprint.obsidian_cost.1;
                self.obsidian.0 += 1;
            }
            Robot::Geode => {
                self.ore.1 -= blueprint.geode_cost.0;
                self.obsidian.1 -= blueprint.geode_cost.1;
                self.geode.0 += 1
            }
        }
    }
    fn accumulate(&mut self) {
        self.ore.1 += self.ore.0;
        self.clay.1 += self.clay.0;
        self.obsidian.1 += self.obsidian.0;
        self.geode.1 += self.geode.0;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
}

impl State {
    fn successors(&self, blueprint: &BluePrint) -> Vec<State> {
        let mut succs = Vec::new();
        if self.minutes == 0 {
            succs
        } else {
            for robot in vec![Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode].iter() {
                if self.inventory.can_afford(robot, blueprint) {
                    let mut inventory = self.inventory.clone();
                    inventory.accumulate();
                    inventory.buy_robot(robot, blueprint);
                    succs.push(State { minutes: self.minutes - 1, inventory });
                }
            }
            let mut inventory = self.inventory.clone();
            inventory.accumulate();
            succs.push(State {
                minutes: self.minutes - 1,
                inventory
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
        (self.minutes + self.inventory.geode.0).cmp(&(other.minutes + other.inventory.geode.0))
/*         (self.minutes + 10 * self.inventory.geode.0 + 5 * self.inventory.geode.1 + 3 * self.inventory.obsidian.0)
        .cmp(&(other.minutes + 10 * other.inventory.geode.0 + 5 * self.inventory.geode.1 + 3 * self.inventory.obsidian.0)) */
    }
}

fn quality_score(blueprint: &BluePrint, minutes_lim: usize, sample_size: Option<usize>, pt1: bool) -> usize {
    let mut frontier = BinaryHeap::new();
    let mut max_yield = 0;
    let mut max_yield_sampled = 0;
    frontier.push(State {
        minutes: minutes_lim,
        inventory: Inventory {
            ore: (1, 0),
            clay: (0, 0),
            obsidian: (0, 0),
            geode: (0, 0),
        },
    });
    let mut explored = AHashSet::new();
    while let Some(cur_state) = frontier.pop() {
        if cur_state.inventory.geode.1 > 0 && cur_state.inventory.geode.1 > max_yield {
            println!("{:?}", (explored.len(), &cur_state));
        }
        if let Some(ss) = sample_size {
            if ss == max_yield_sampled {
                break;
            }
        }
        if explored.contains(&cur_state) {
            continue;
        }
        if cur_state.inventory.geode.1 > 0 {
            if cur_state.inventory.geode.1 > max_yield {
                max_yield = cur_state.inventory.geode.1;
                max_yield_sampled = 0;
            } else {
                max_yield_sampled += 1;
            }
        }
        for suc_state in cur_state.successors(blueprint) {
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
        .inspect(|e| println!("{:?}", e))
        .collect_vec()
}

pub fn solve(str: &str) -> (usize, usize) {
    let parsed = parse(str);
    let s1 = parsed.iter()
        .map(|bp| quality_score(bp, 24, None, true))
        .inspect(|ql| println!("quality-score: {:?}", ql))
        .sum();
    let s2 = parsed.iter()
        .take(3)
        .map(|bp| quality_score(bp, 32, Some(100_000_000), false))
        .inspect(|ql| println!("max-yield: {:?}", ql))
        .reduce(|x, y| x * y).unwrap();
    (s1, s2)
}
