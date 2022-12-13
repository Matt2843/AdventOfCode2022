use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq, Clone)]
enum NestedDevil {
    U(usize),
    V(Vec<NestedDevil>),
}

impl PartialOrd for NestedDevil {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NestedDevil {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (NestedDevil::U(sv), NestedDevil::U(ov)) => sv.cmp(ov),
            (NestedDevil::V(sv), NestedDevil::V(ov)) => {
                for (left, right) in sv.iter().zip(ov) {
                    match left.cmp(right) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return  Ordering::Greater,
                        _ => () // when they're equal we continue
                    }
                }
                // we ran out of left elements to check, in the zip..
                // so we just need to see if sv.len() <= ov.len() -> Less
                sv.len().cmp(&ov.len())
            },
            (NestedDevil::U(sv), ov) => NestedDevil::V(vec![NestedDevil::U(*sv)]).cmp(ov),
            (sv, NestedDevil::U(ov)) => sv.cmp(&NestedDevil::V(vec![NestedDevil::U(*ov)])),
        }
    }
}

fn crunch(s: &str) -> Vec<String> {
    let mut strs = Vec::new();
    let mut chars = Vec::new();
    let mut depth = 0;
    for ch in s.chars() {
        match (depth, ch) {
            (0, '[') => depth += 1,
            (1, ']') => {
                if !chars.is_empty() {
                    strs.push(chars.drain(..).collect());
                }
                return strs;
            }
            (1, ',') => strs.push(chars.drain(..).collect()),
            (_, ch) => {
                chars.push(ch);
                match ch {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => ()
                }
            }
        }
    }
    panic!("failed to crunch {s}")
}

fn parse_packet(packet_str: &str) -> NestedDevil {
    match packet_str.starts_with('[') {
        true => NestedDevil::V(crunch(packet_str).iter().map(|t| parse_packet(t)).collect()),
        false => NestedDevil::U(packet_str.parse().unwrap())
    }
}

pub fn solve(str: &str) -> (usize, usize) {
    let parsed_packets = 
        str.split("\n\n")
        .map(|l| l.split_once("\n").unwrap())
        .map(|(l, r)| (parse_packet(l), parse_packet(r)))
        //.inspect(|(l, r)| println!("\n{:?}\n{:?}", l, r))
        .collect_vec();
    let s1 = parsed_packets
        .iter()
        .enumerate()
        .flat_map(|(i, (left, right))| match left < right {
            true => Some(i + 1),
            false => None
        }).sum();
    let divider_vec = vec![parse_packet("[[2]]"), parse_packet("[[6]]")];
    let sorted = parsed_packets.iter()
        .fold(BinaryHeap::from_iter(divider_vec.iter()), |mut acc, (left, right)| {
            acc.push(&left);
            acc.push(&right);
            acc
        }).into_sorted_vec();
    let s2 = (sorted.binary_search(&&divider_vec[0]).unwrap() + 1) * (sorted.binary_search(&&divider_vec[1]).unwrap() + 1);
    (s1,s2)
}