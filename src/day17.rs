use std::clone;

use itertools::Itertools;

enum Shape {
    HorLine,
    Plus,
    L,
    VerLine,
    Square,
}

fn spawn_rock(tetris_map: &mut Vec<Vec<char>>, rock: &Shape) -> Vec<(usize, usize)> {
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
            let mut positions = (c_offset..c_offset+3).map(|c| (r_offset-1, c)).collect_vec();
            positions.push((r_offset, c_offset +1));
            positions.push((r_offset-2, c_offset +1));
            positions
        },
        Shape::L => {
            tetris_map[r_offset][c_offset..c_offset+3].iter_mut().for_each(|c| *c = '#');
            tetris_map[r_offset-1][c_offset + 2] = '#';
            tetris_map[r_offset-2][c_offset + 2] = '#';
            let mut positions = (c_offset..c_offset+3).map(|c| (r_offset, c)).collect_vec();
            positions.push((r_offset-1, c_offset +2));
            positions.push((r_offset-2, c_offset +2));
            positions
        },
        Shape::VerLine => {
            tetris_map[r_offset-3..=r_offset].iter_mut().for_each(|r| r[c_offset] = '#');
            (r_offset-3..=r_offset).map(|r| (r, c_offset)).collect_vec()
        }
        Shape::Square => {
            tetris_map[r_offset-1..=r_offset].iter_mut().for_each(|r| r[c_offset..c_offset + 2].iter_mut().for_each(|c| *c = '#'));
            let mut positions = Vec::new();
            positions.push((r_offset-1, c_offset));
            positions.push((r_offset-1, c_offset+1));
            positions.push((r_offset, c_offset));
            positions.push((r_offset, c_offset+1));
            positions
        }
    }
}

#[derive(PartialEq, Eq)]
enum Move {
    Right,
    Left,
    Down,
}

fn valid_move(tetris_map: &mut Vec<Vec<char>>, rock: &Vec<(usize, usize)>, direction: &Move) -> bool {
    match direction {
        Move::Left => {
            let to_check = rock.into_iter()
                .group_by(|(r, _)| *r)
                .into_iter()
                .map(|(_, group)| {
                    group.into_iter().min_by(|(_, c1), (_, c2)| c1.cmp(c2)).unwrap()
                })
                .collect_vec();
            !to_check.iter().any(|(r,c)| *c == 0 || tetris_map[*r][*c-1] == '#')
        },
        Move::Right => {
            let to_check = rock.into_iter()
                .group_by(|(r, _)| *r)
                .into_iter()
                .map(|(_, group)| {
                    group.into_iter().max_by(|(_, c1), (_, c2)| c1.cmp(c2)).unwrap()
                })
                .collect_vec();
            !to_check.iter().any(|(r,c)| *c == tetris_map[*r].len()-1 || tetris_map[*r][*c+1] == '#')
        },
        Move::Down => {
            let to_check = rock.into_iter()
                .into_group_map_by(|(_, c)| *c)
                .iter().map(|(c, r)| (r.iter().map(|r| r.0).max().unwrap(), *c))
                .collect_vec();
            !to_check.iter().any(|(r,c)| tetris_map[*r+1][*c] == '#')
        }
    }
}

fn move_rock(tetris_map: &mut Vec<Vec<char>>, rock: &mut Vec<(usize, usize)>, direction: Move) -> bool {
    if !valid_move(tetris_map, &rock, &direction) {
        return false;
    }
    let mut translated_rock = rock.clone();
    for (r, c) in translated_rock.iter_mut() {
        match direction {
            Move::Left => {
                *c -= 1;
            },
            Move::Right => {
                *c += 1;
            }
            Move::Down => {
                *r += 1;
            }
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

fn print_tetris(tetris_map: &mut Vec<Vec<char>>) {
    for x in tetris_map[2022*10-20..2022*10].iter() {
        for y in x.iter() {
            print!("{}", y);
        }
        println!();
    }
    println!();
}

fn direction(c: char) -> Move {
    match c {
        '<' => Move::Left,
        '>' => Move::Right,
        _ => unreachable!()
    }
}

pub fn solve(str: &str) -> (usize, usize) {
    let mut tetris_map = vec![vec!['.';7];2022*10];
    let len = tetris_map.len();
    tetris_map[len-1].iter_mut()
        .for_each(|c| *c = '#');
    let shape_rota = vec![Shape::HorLine, Shape::Plus, Shape::L, Shape::VerLine, Shape::Square];
    let mut shape_iter = shape_rota.iter().cycle();
    let mut directions = str.trim().chars().into_iter().cycle();
    for x in 0..2022 as usize {
        let next_rock_shape = shape_iter.next().unwrap();
        let mut next_rock = spawn_rock(&mut tetris_map, next_rock_shape);
        //print_tetris(&mut tetris_map);
        loop {
            move_rock(&mut tetris_map, &mut next_rock, direction(directions.next().unwrap()));
            match move_rock(&mut tetris_map, &mut next_rock, Move::Down) {
                true => (),
                false => break
            }
        }
        println!("{}/1000000000000", x);
    }
    let highest_rock_r = tetris_map.len() - tetris_map.iter().enumerate().find(|(_, r)| r.contains(&'#')).unwrap().0;
    (highest_rock_r - 1, 0)
}