use itertools::Itertools;
use ahash::AHashMap;

enum Shape {
    HorLine,
    Plus,
    L,
    VerLine,
    Square,
}

fn spawn_rock(tetris_map: &mut [Vec<char>], rock: &Shape) -> Vec<(usize, usize)> {
    let highest_rock_r = tetris_map.iter().enumerate().find(|(_, r)| r.contains(&'#')).unwrap().0;
    let c_offset = 2;
    let r_offset = highest_rock_r - 4;
    match rock {
        Shape::HorLine => {
            tetris_map[r_offset][c_offset..c_offset+4].iter_mut().for_each(|c| *c = '#');
            (c_offset..c_offset+4).map(|c| (r_offset, c)).collect_vec()
        },
        Shape::Plus => {
            tetris_map[r_offset][c_offset + 1] = '#';
            tetris_map[r_offset-1][c_offset..c_offset+3].iter_mut().for_each(|c| *c = '#');
            tetris_map[r_offset-2][c_offset + 1] = '#';
            vec![(r_offset, c_offset+1),(r_offset-1,c_offset),(r_offset-1,c_offset+1),(r_offset-1, c_offset+2),(r_offset-2, c_offset+1)]
        },
        Shape::L => {
            tetris_map[r_offset][c_offset..c_offset+3].iter_mut().for_each(|c| *c = '#');
            tetris_map[r_offset-1][c_offset + 2] = '#';
            tetris_map[r_offset-2][c_offset + 2] = '#';
            vec![(r_offset,c_offset),(r_offset,c_offset+1),(r_offset,c_offset+2),(r_offset-1,c_offset+2),(r_offset-2,c_offset+2)]
        },
        Shape::VerLine => {
            tetris_map[r_offset-3..=r_offset].iter_mut().for_each(|r| r[c_offset] = '#');
            (r_offset-3..=r_offset).map(|r| (r, c_offset)).collect_vec()
        }
        Shape::Square => {
            tetris_map[r_offset-1..=r_offset].iter_mut().for_each(|r| r[c_offset..c_offset + 2].iter_mut().for_each(|c| *c = '#'));
            vec![(r_offset-1, c_offset),(r_offset-1, c_offset+1),(r_offset, c_offset),(r_offset, c_offset+1)]
        }
    }
}

#[derive(PartialEq, Eq)]
enum Move {
    Right,
    Left,
    Down,
}

fn valid_move(tetris_map: &[Vec<char>], rock: &[(usize, usize)], direction: &Move) -> bool {
    match direction {
        Move::Left => !rock.iter()
                .into_group_map_by(|(r, _)| *r)
                .iter().map(|(r, c)| (*r, c.iter().map(|c| c.1).min().unwrap()))
                .any(|(r,c)| c == 0 || tetris_map[r][c-1] == '#'),
        Move::Right => !rock.iter()
                .into_group_map_by(|(r, _)| *r)
                .iter().map(|(r, c)| (*r, c.iter().map(|c| c.1).max().unwrap()))
                .any(|(r,c)| c == tetris_map[r].len()-1 || tetris_map[r][c+1] == '#'),
        Move::Down => !rock.iter()
                .into_group_map_by(|(_, c)| *c)
                .iter().map(|(c, r)| (r.iter().map(|r| r.0).max().unwrap(), *c))
                .any(|(r,c)| tetris_map[r+1][c] == '#')
    }
}

fn move_rock(tetris_map: &mut [Vec<char>], rock: &mut Vec<(usize, usize)>, direction: Move) -> bool {
    if !valid_move(tetris_map, rock, &direction) {
        return false;
    }
    let mut translated_rock = rock.clone();
    for (r, c) in translated_rock.iter_mut() {
        match direction {
            Move::Left => *c -= 1,
            Move::Right => *c += 1,
            Move::Down => *r += 1,
        }
    }
    for (row, col) in rock.iter() {
        tetris_map[*row][*col] = '.';
    }
    for (row, col) in translated_rock.iter() {
        tetris_map[*row][*col] = '#';
    }
    *rock = translated_rock;
    true
}

pub fn solve(str: &str) -> (usize, usize) {
    let mut tetris_map = vec![vec!['.';7];2022*10];
    let len = tetris_map.len();
    tetris_map[len-1].iter_mut().for_each(|c| *c = '#');
    let shape_rota = vec![Shape::HorLine, Shape::Plus, Shape::L, Shape::VerLine, Shape::Square];
    let mut shape_iter = shape_rota.iter().enumerate().cycle();
    let mut directions = str.trim().chars().into_iter().enumerate().cycle();
    let mut i = 0;
    let mut s1 = 0;
    let mut max_y = 0;
    let mut added_height = 0;
    let mut cycle_map = AHashMap::new();
    let n: usize = 1_000_000_000_000;
    while i < n {
        if i == 2022 {
            s1 = max_y;
        }
        let (shape_index, next_rock_shape) = shape_iter.next().unwrap();
        let mut next_rock = spawn_rock(&mut tetris_map, next_rock_shape);
        loop {
            let (dir_index, dir) = directions.next().unwrap();
            move_rock(&mut tetris_map, &mut next_rock, if dir == '<' {Move::Left} else {Move::Right});
            match move_rock(&mut tetris_map, &mut next_rock, Move::Down) {
                true => (),
                false =>  {
                    max_y = tetris_map.len() - tetris_map.iter().enumerate().find(|(_, r)| r.contains(&'#')).unwrap().0 - 1;
                    break;
                }
            }
            if i > 2022 {
                let last_n = tetris_map.iter().skip(tetris_map.len() - max_y).take(10).cloned().collect_vec();
                let cycle_key = (dir_index, shape_index, last_n);
                if let Some((last_i, last_max_y)) = cycle_map.get(&cycle_key) {
                    let dy = max_y - last_max_y;
                    let di = i - last_i;
                    let times = (n-i).div_euclid(di);
                    added_height += times * dy;
                    i += times * di;
                }
                cycle_map.insert(cycle_key, (i, max_y));
            }
        }
        i += 1;
    }
    (s1, max_y + added_height)
}