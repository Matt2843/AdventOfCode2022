
use itertools::Itertools;

fn parse_cave(str: &str, add_floor: bool) -> (Vec<Vec<char>>, usize, usize) {
    let mut parsed = str.trim().split("\n")
        .map(|l| l.trim().split(" -> ")
            .map(|e| e.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .collect_vec()
        ).collect_vec();

    let all_points = parsed.iter()
        .flat_map(|c| c)
        .collect_vec();
    let max_x = all_points.iter().map(|p| p.0).max().unwrap();
    let max_y = all_points.iter().map(|p| p.1).max().unwrap();

    if add_floor {
        let mut floor_seg = Vec::new();
        floor_seg.push((0, max_y+2));
        floor_seg.push((max_x*10, max_y+2));
        parsed.push(floor_seg);
    }

    let cave = parsed.iter()
        .fold(vec![vec!['.';max_x*10+1];max_y+3], |mut acc, points| {
            for window in points.windows(2) {
                let mut ord_window = vec![window[0],window[1]];
                ord_window.sort();
                for y in ord_window[0].1..=ord_window[1].1 {
                    for x in ord_window[0].0..=ord_window[1].0 {
                        acc[y][x] = '#';
                    }
                }
            }
            acc
        });
    (cave, max_x, max_y)
}

fn simulate_sand(cave: &mut [Vec<char>], (max_width, max_depth): (Option<usize>, Option<usize>)) -> usize {
    let mut iterations = 0;
    loop {
        let mut void = false;
        let mut sand: (usize, usize) = (0, 500);
        loop {
            sand = (sand.0+1, sand.1);
            if max_width.is_some() && (sand.0 > max_depth.unwrap() || sand.1 > max_width.unwrap()) {
                void = true;
                break;
            }
            if cave[sand.0][sand.1] != '.' {
                if cave[sand.0][sand.1-1] == '.' {
                    sand = (sand.0, sand.1-1);
                } else if cave[sand.0][sand.1+1] == '.' {
                    sand = (sand.0, sand.1+1);
                } else {
                    if max_width.is_none() && sand.0-1 == 0 && sand.1 == 500 {
                        void = true;
                        break;
                    }
                    cave[sand.0-1][sand.1] = 'o';
                    break;
                }
            }
        }
        if max_width.is_none() {
            iterations += 1;
        }
        if void {
            break;
        }
        if max_width.is_some() {
            iterations += 1;
        }
    }
    iterations
}

pub fn solve(str: &str) -> (usize, usize) {
    let (mut cave, max_width, max_depth) = parse_cave(str, false);
    let s1 = simulate_sand(&mut cave, (Some(max_width), Some(max_depth)));
    let (mut cave2, _, _) = parse_cave(str, true);
    let s2 = simulate_sand(&mut cave2, (None, None));
    (s1, s2)
}