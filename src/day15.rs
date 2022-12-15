use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

struct Sensor {
    pos: (i32, i32),
    c_beacon: (i32, i32),
    range: i32,
}

fn parse(str: &str) -> Vec<Sensor> {
    let line_gex = Regex::new(r"^Sensor.*?x=(-?\d+),\sy=(-?\d+).*?x=(-?\d+),\sy=(-?\d+)$").unwrap();
    str.trim()
        .lines()
        .map(|l| line_gex.captures(l.trim()).unwrap())
        .map(|m| Sensor {
            pos: (
                m.get(1).unwrap().as_str().parse().unwrap(),
                m.get(2).unwrap().as_str().parse().unwrap(),
            ),
            c_beacon: (
                m.get(3).unwrap().as_str().parse().unwrap(),
                m.get(4).unwrap().as_str().parse().unwrap(),
            ),
            range: manhatten_dist(
                (
                    m.get(1).unwrap().as_str().parse().unwrap(),
                    m.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    m.get(3).unwrap().as_str().parse().unwrap(),
                    m.get(4).unwrap().as_str().parse().unwrap(),
                ),
            ),
        })
        .collect_vec()
}

fn manhatten_dist(from: (i32, i32), to: (i32, i32)) -> i32 {
    let dx = from.0.abs_diff(to.0);
    let dy = from.1.abs_diff(to.1);
    dx as i32 + dy as i32
}

fn coverage(sensor: &Sensor, y: i32) -> HashSet<(i32, i32)> {
    let mut coverage = HashSet::new();
    for x in (sensor.pos.0 - sensor.range)..(sensor.pos.0 + sensor.range) {
        if sensor.c_beacon != (x, y) && manhatten_dist(sensor.pos, (x, y)) <= sensor.range {
            coverage.insert((x, y));
        }
    }
    coverage
}

fn distress_tuning_freq(sensors: &[Sensor]) -> usize {
    let mut ranges = Vec::with_capacity(sensors.len());
    for y in 0..=4000000 as i32 {
        ranges.clear();
        sensors.iter()
            .filter(|s| y.abs_diff(s.pos.1) < s.range as u32)
            .map(|s| {
                let dx = s.range - s.pos.1.abs_diff(y) as i32;
                (s.pos.0 - dx, s.pos.0 + dx)
            })
            .for_each(|s| ranges.push(s));
        let mut x = 0;
        loop {
            let prev = x;
            x = ranges.iter()
                .filter(|s| s.1 >= x)
                .fold(x, |acc, s| {
                    if (s.0..=s.1).contains(&acc) {
                        return (s.1 + 1).max(acc);
                    }
                    acc
                });
            if x == prev {
                return x as usize * 4000000 + y as usize;
            } else if x >= 4000000 {
                break;
            }
        }
    }
    unreachable!()
}

pub fn solve(str: &str) -> (usize, usize) {
    let sensors = parse(str);
    let s1 = sensors
        .iter()
        .map(|s| coverage(s, 2000000))
        .fold(HashSet::new(), |mut acc, x| {
            acc.extend(x);
            acc
        }).len();
    let s2 = distress_tuning_freq(&sensors);
    (s1, s2)
}
