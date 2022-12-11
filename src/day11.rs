use std::collections::{BTreeMap, VecDeque};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
struct Monkey {
    monkey_id: usize,
    items: VecDeque<usize>,
    operation: (char, Option<usize>),
    test: (usize, usize, usize),
    inspection_count: usize,
}

impl Monkey {
    fn new(str: &str) -> Monkey {
        let info = str.lines().into_iter().map(|l| l.trim()).join("\n");
        let monkey_gex: Regex = Regex::new(r"^Monkey\s(\d+):\nStarting.*?((\d+,?\s?)+)\nOperation.*?old\s(.*)\nTest.*?(\d+)\nIf\strue.*?(\d+)\nIf\sfalse.*?(\d+)$").unwrap();
        match monkey_gex.captures(info.as_str()) {
            Some(monkey) => {
                let op = monkey.get(4).unwrap().as_str().split_once(' ').unwrap();
                Monkey {
                    monkey_id: monkey.get(1).unwrap().as_str().parse().unwrap(),
                    items: monkey.get(2).unwrap().as_str().split(", ").map(|item| item.parse::<usize>().unwrap())
                        .fold(VecDeque::new(), |mut acc, item| { 
                            acc.push_back(item);
                            acc
                        }),
                    operation: (op.0.parse().unwrap(), op.1.trim().parse().ok()),
                    test: (monkey.get(5).unwrap().as_str().parse().unwrap(),
                           monkey.get(6).unwrap().as_str().parse().unwrap(),
                           monkey.get(7).unwrap().as_str().parse().unwrap()),
                    inspection_count: 0,
                }
            }
            None => unreachable!()
        }
    }

    fn inspect(&mut self, divisors_product: &usize, pt1: bool) -> Vec<(usize, usize)> {
        let mut d_items = Vec::new();
        while !self.items.is_empty() {
            let inspection_item = &self.items.pop_front().unwrap();
            let mut worry_level = match &self.operation {
                ('+', val) => {
                    match val {
                        Some(val) => inspection_item + val,
                        None => inspection_item + inspection_item,
                    }
                }
                ('*', val) => {
                    match val {
                        Some(val) => inspection_item * val,
                        None => inspection_item * inspection_item,
                    }
                }
                _ => unreachable!()
            };
            if pt1 {
                worry_level = worry_level.div_euclid(3);
            }
            worry_level %= divisors_product;
            if worry_level % self.test.0 == 0 {
                d_items.push((worry_level, self.test.1));
            } else {
                d_items.push((worry_level, self.test.2));
            }
            self.inspection_count += 1;
        }
        d_items
    }
}

fn slv(monkeys: &mut BTreeMap<usize, Monkey>, divisors_product: &usize, pt1: bool) -> usize {
    for _ in 0..if pt1 {20} else {10000} {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(&i).unwrap();
            let inspected_items = monkey.inspect(divisors_product, pt1);
            for (item, next_monkey) in inspected_items {
                monkeys.get_mut(&next_monkey).unwrap().items.push_back(item);
            }
        }
    }
    let mut insp_counts = monkeys.values().map(|m| m.inspection_count).sorted().rev();
    insp_counts.next().unwrap() * insp_counts.next().unwrap()
}

pub fn solve(str: &str) -> (usize, usize) {
    let mut monkeys = str.split("\n\n").map(Monkey::new)
        .fold(BTreeMap::new(), |mut acc, monkey| {
            acc.entry(monkey.monkey_id).or_insert(monkey);
            acc
        });
    let divisors_product = monkeys.values()
        .into_iter().map(|m| m.test.0)
        .reduce(|x, y| x * y).unwrap();
    (slv(&mut monkeys.clone(), &divisors_product, true), slv(&mut monkeys, &divisors_product, false))
}