use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solve(str: &str) -> (String, String) {
    let mut input_spl =
        include_str!(r"C:\Users\mathh\source\repos\AdventOfCode2022\aoc_rust\input\2022-5.txt")
            .split("\n\n");

    let mut vecdeques = HashMap::new();

    for i in 0..9 {
        vecdeques.insert(i as usize, VecDeque::<char>::new());
    }

    input_spl
        .nth(0)
        .unwrap()
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

    let moves = input_spl
        .nth(0)
        .unwrap()
        .split("\n")
        .map(|l| l.split_ascii_whitespace().collect_vec())
        .map(|l| {
            (
                l[1].parse::<usize>().unwrap(),
                l[3].parse::<usize>().unwrap(),
                l[5].parse::<usize>().unwrap(),
            )
        })
        .collect_vec();

    for (quant, from, to) in moves {
        for i in 0..quant {
            let x = vecdeques.get_mut(&(from - 1)).unwrap().pop_back().unwrap();
            vecdeques.get_mut(&(to - 1)).unwrap().push_back(x);
        }
    }

    let mut s = String::new();
    for i in 0..9 {
        s.push(*vecdeques.get(&i).unwrap().back().unwrap());
    }
    (s, "".to_string())
}
