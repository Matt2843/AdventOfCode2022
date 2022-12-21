use ahash::AHashMap;
use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    action: Action,
}

#[derive(Debug, Clone)]
enum Action {
    Yell(i64),
    Calc((String, Operation, String)),
}

#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
}

fn parse(str: &str) -> AHashMap<String, Monkey> {
    let vec_splits = str.trim()
        .lines()
        .map(|l| l.trim().split(' ').enumerate().collect_vec())
        .collect_vec();
    let mut monkeys = AHashMap::new();
    for vs in vec_splits {
        let name = vs[0].1.trim_matches(':').to_string();
        let nxt = vs[1].1.parse::<i64>();
        match nxt {
            Ok(v) => {
                monkeys.entry(name.clone()).or_insert(Monkey {name, action: Action::Yell(v)});
            },
            Err(_) => {
                let lhs = vs[1].1.trim().to_string();
                let op = vs[2].1.trim();
                let op = match op {
                    "+" => Operation::Plus,
                    "-" => Operation::Minus,
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    _ => unreachable!()
                };
                let rhs = vs[3].1.trim().to_string();
                monkeys.entry(name.clone()).or_insert(Monkey {name, action: Action::Calc((lhs, op, rhs))});
            }
        }
    }
    monkeys
}

fn eval(name: &String, map: &AHashMap<String, Monkey>) -> i64 {
    let val = match &map[name].action {
        Action::Yell(v) => *v,
        Action::Calc((lhs, op, rhs)) => {
            let lhs = lhs.parse::<i64>().unwrap_or(eval(lhs, map));
            let rhs = rhs.parse::<i64>().unwrap_or(eval(rhs, map));
            match *op {
                Operation::Equal => lhs,
                Operation::Plus => lhs + rhs,
                Operation::Minus => lhs - rhs,
                Operation::Multiply => lhs * rhs,
                Operation::Divide => lhs / rhs,
            }
        }
    };
    val
}

fn binary_search_yell(monkeys: &mut AHashMap<String, Monkey>) -> usize {
    let mut root = monkeys[&"root".to_string()].clone();
    match root.action {
        Action::Calc((lhs, _, rhs)) => { root.action = Action::Calc((lhs, Operation::Equal, rhs)) },
        _ => unreachable!()
    }
    monkeys.insert(root.name.clone(), root);
    let mut low = 0i64;
    let mut high = 100_000_000_000_000i64;
    loop {
        let yell = (low + high) / 2;
        let mut nxt_monkey = monkeys[&"humn".to_string()].clone();
        match nxt_monkey.action {
            Action::Yell(_) => {
                nxt_monkey.action = Action::Yell(yell);
            }
            _ => unreachable!()
        }
        monkeys.insert(nxt_monkey.name.clone(), nxt_monkey);
        let x = eval(&"root".to_string(), &monkeys);
        match x.cmp(&23622695042414) {
            Ordering::Less => high = yell + 1,
            Ordering::Greater => low = yell - 1,
            Ordering::Equal => { return yell as usize; }
        }
    }
}

pub fn solve(str: &str) -> (usize, usize) {
    let mut monkeys = parse(str);
    let s1 = eval(&"root".to_string(), &monkeys);
    (s1 as usize, binary_search_yell(&mut monkeys))
}
