use std::collections::{BTreeSet, HashMap, BTreeMap};
use itertools::Itertools;
use regex::Regex;

fn parse(str: &str) -> (HashMap<String, usize>, HashMap<String, Vec<String>>) {
    let valve_gex = Regex::new(r"^Valve\s(\w{2}).*?=(\d+).*?valves?\s((\w{2},?\s?)+)$").unwrap();
    str.lines()
        .map(|l| valve_gex.captures(l).expect("valve_gex didn't match {l}"))
        .map(|c| 
            (c.get(1).unwrap().as_str().trim().to_string(),
            c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            c.get(3).unwrap().as_str().split(", ").map(|spl| spl.trim().to_string()).collect_vec(),
        ))
        .fold((HashMap::new(), HashMap::new()), 
        |(mut flow_rates, 
            mut connections), 
            (id, flow, conns)| {
                flow_rates.entry(id.clone()).or_insert(flow);
                connections.entry(id).or_insert(conns);
            (flow_rates, connections)
        })
}

fn brute_force(cache: &mut BTreeMap<(String, usize, BTreeSet<String>, bool), usize>,
    flow_rates: &HashMap<String,usize>, connections: &HashMap<String, Vec<String>>, 
    id: &String, remaining: usize, opened_valves: &BTreeSet<String>, let_elephant: bool) -> usize 
{
    if let Some(cached_max_flow) = cache.get(&(id.clone(), remaining, opened_valves.clone(), let_elephant)) {
        return *cached_max_flow;
    }
    if remaining == 0 {
        if let_elephant {
            return brute_force(cache, flow_rates, connections, &"AA".to_string(), 26, opened_valves, false);
        }
        return 0;
    }
    let mut max_flow = connections[id].iter()
        .map(|next| brute_force(cache, flow_rates, connections, next, remaining - 1, opened_valves, let_elephant))
        .max().unwrap();
    
    let c_flow = flow_rates[id];
    if c_flow > 0 && !opened_valves.contains(id) {
        let mut new_valves = opened_valves.clone();
        new_valves.insert(id.clone());
        max_flow = std::cmp::max(max_flow, 
            (remaining - 1) * c_flow + brute_force(cache, flow_rates, connections, id, remaining - 1, &new_valves, let_elephant)
        );
    }
    cache.insert((id.clone(), remaining, opened_valves.clone(), let_elephant), max_flow);
    max_flow
}

pub fn solve(str: &str) -> (usize, usize) {
    let (flow_rates, connections) = parse(str);
    (brute_force(&mut BTreeMap::new(), &flow_rates, &connections,&"AA".to_string(), 30, &mut BTreeSet::new(), false),
    brute_force(&mut BTreeMap::new(), &flow_rates, &connections,&"AA".to_string(), 26, &mut BTreeSet::new(), true))
}