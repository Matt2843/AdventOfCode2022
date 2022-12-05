use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

fn parse_stacks(first_in: &str) -> HashMap::<usize, VecDeque<char>> {
    let mut vecdeques = HashMap::new();
    for i in 0..9 {
        vecdeques.insert(i as usize, VecDeque::<char>::new());
    }
    first_in
        .split("\n")
        .map(|l| l.chars().chunks(4))
        .for_each(|x| {
            x.into_iter()
                .enumerate()
                .map(|(i, c)| (i, c.into_iter().nth(1).unwrap()))
                .for_each(|(i, c)| {
                    if c != ' ' && !c.is_numeric() {
                        vecdeques.get_mut(&i).unwrap().push_back(c)
                    }
                })
        });
    vecdeques
}

fn parse_moves(input_two: &str) -> Vec<(usize, usize, usize)> {
        input_two.split("\n")
        .map(|l| l.split_ascii_whitespace().collect_vec())
        .map(|l| {
            (
                l[1].parse::<usize>().unwrap(),
                l[3].parse::<usize>().unwrap(),
                l[5].parse::<usize>().unwrap(),
            )
        })
        .collect_vec()
}

fn get_result(deques: HashMap<usize, VecDeque<char>>) -> String {
    let mut s = String::new();
    for i in 0..deques.len() {
        if deques.get(&i).unwrap().is_empty() {
            continue;
        }
        s.push(*deques.get(&i).unwrap().front().unwrap());
    }
    s
}

pub fn solve(str: &str) -> (String, String) {
    let mut input_spl = str.split("\n\n");
    let mut vecdeques = parse_stacks(input_spl.nth(0).unwrap());
    let moves = parse_moves(input_spl.nth(0).unwrap());

    let mut s2_deques = vecdeques.clone();
    for (quant, from, to) in moves {
        let mut z = Vec::<char>::new();
        for _ in 0..quant {
            let x = vecdeques.get_mut(&(from - 1)).unwrap().pop_front().unwrap();
            let y = s2_deques.get_mut(&(from - 1)).unwrap().pop_front().unwrap();
            vecdeques.get_mut(&(to - 1)).unwrap().push_front(x);
            z.push(y);
        }
        z.reverse();
        for i in 0..z.len() {
            s2_deques.get_mut(&(to - 1)).unwrap().push_front(*z.get(i).unwrap());
        }
    }
    (get_result(vecdeques), get_result(s2_deques))
}
