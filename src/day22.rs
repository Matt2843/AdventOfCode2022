use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(char),
}

#[derive(Debug, Clone)]
struct Map {
    location: (usize, usize),
    direction: usize,
    coords: Vec<Vec<char>>,
}

impl Map {
    // hard-code the edge-transitions..
    fn next_location(&self) -> (usize, usize, usize) {
        match (self.location.0, self.location.1, self.direction) { // row, col, direction
            // region 1 special cases
            // travel north from region 1 -> enter from the west side of region 6
            (0, 50..=99, 3) => (self.location.1 + 100, 0, 0),
            // travel west from region 1 -> enter from the west side flipped of region 5
            (0..=49, 50, 2) => (149 - self.location.0, 0, 0),

            // region 2 special cases NORTH; EAST; SOUTH;
            // travel north from region 2 -> enter from south of region 6
            (0, 100..=149, 3) => (199, self.location.1 - 100, 3),
            // travel east from region 2 -> enter from east of region 4
            (0..=49, 149, 0) => (149 - self.location.0, 99, 2),
            // travel south from region 2 -> enter from east of region 3
            (49, 100..=149, 1) => (self.location.1 - 50, 99, 2),

            // region 3 special cases EAST, WEST
            // travel east from region 3 -> enter from south of region 2
            (50..=99, 99, 0) => (49, self.location.0 + 50, 3),
            // travel west from region 3 -> enter from north of region 5
            (50..=99, 50, 2) => (100, self.location.0 - 50, 1),

            // region 4 special cases EAST, SOUTH
            // travel east from region 4 -> enter from east of region 2
            (100..=149, 99, 0) => (149 - self.location.0, 149, 2),
            // travel south from region 4 -> enter from east of region 6
            (149, 50..=99, 1) => (self.location.1 + 100, 49, 2),

            // region 5 special cases NORTH, WEST
            // travel north from region 5 -> enter from west of region 3
            (100, 0..=49, 3) => (self.location.1 + 50, 50, 0),
            // travel west from region 5 -> enter from west of region 1
            (100..=149, 0, 2) => (149 - self.location.0, 50, 0),

            // region 6 sepcial cases EAST, SOUTH, WEST
            // travel east from region 6 -> enter from south of region 4
            (150..=199, 49, 0) => (149, self.location.0 - 100, 3),
            // travel south from region 6 -> enter from north of region 2
            (199, 0..=49, 1) => (0, self.location.1 + 100, 1),
            // travel west from region 6 -> enter from north of region 1
            (150..=199, 0, 2) => (0, self.location.0 - 100, 1),

            _ => match self.direction {
                0 => (self.location.0, self.location.1 + 1, self.direction),
                1 => (self.location.0 + 1, self.location.1, self.direction),
                2 => (self.location.0, self.location.1 - 1, self.direction),
                3 => (self.location.0 - 1, self.location.1, self.direction),
                _ => unreachable!()
            }
        }
    }
    fn eval_instruction_cube(&mut self, instruction: &Instruction) {
        //println!("{:?}", (instruction, self.location, self.direction));
        match instruction {
            Instruction::Move(units) => {
                for _ in 0..*units {
                    let (cr, cc, next_dir) = self.next_location();
                    match self.coords[cr][cc] {
                        '#' => break,
                        '.' => { self.location = (cr, cc); self.direction = next_dir },
                        _ => unreachable!()
                    }
                }
            }
            Instruction::Turn(direction) => match direction {
                'R' => self.direction = (self.direction + 1) % 4,
                'L' => self.direction = (self.direction + 3) % 4,
                _ => unreachable!(),
            }
        }
    }
    fn eval_instruction(&mut self, instruction: &Instruction) {
/*         println!(
            "evaluating {:?}",
            (instruction, self.direction, self.location)
        ); */
        //self.print();
        match instruction {
            Instruction::Move(units) => {
                let (mut cr, mut cc) = self.location;
                for _ in 0..*units {
                    match self.direction {
                        0 => {
                            // move right
                            if cc + 1 == self.coords[cr].len() {
                                cc = self.coords[cr]
                                    .iter()
                                    .position(|c| !c.is_whitespace())
                                    .unwrap();
                            } else {
                                cc += 1;
                            }
                        }
                        1 => {
                            // move down
                            if cr + 1 == self.coords.len()
                                || self.coords[cr + 1].len() <= cc
                                || self.coords[cr + 1][cc].is_whitespace()
                            {
                                cr = self.coords
                                    .iter()
                                    .position(|r| r.len() > cc && !r[cc].is_whitespace())
                                    .unwrap();
                            } else {
                                cr += 1;
                            }
                        }
                        2 => {
                            // move left
                            if cc as i32 - 1 < 0 || self.coords[cr][cc - 1].is_whitespace() {
                                cc = self.coords[cr].len() - 1;
                            } else {
                                cc -= 1;
                            }
                        }
                        3 => {
                            // move up
                            if cr as i32 - 1 < 0 || self.coords[cr - 1][cc].is_whitespace() {
                                // circle around from bot.
                                cr = self.coords
                                    .iter()
                                    .positions(|r| r.len() > cc)
                                    .last().unwrap();
                            } else {
                                cr -= 1;
                            }
                        },
                        _ => unreachable!(),
                    }
                    match self.coords[cr][cc] {
                        '#' => break,
                        '.' | '>' | 'v' | '<' | '^' => {
                            self.location = (cr, cc);
                            match self.direction {
                                0 => self.coords[cr][cc] = '>',
                                1 => self.coords[cr][cc] = 'v',
                                2 => self.coords[cr][cc] = '<',
                                3 => self.coords[cr][cc] = '^',
                                _ => unreachable!()
                            }
                        },
                        _ => unreachable!(),
                    }
                }
            }
            Instruction::Turn(direction) => match direction {
                'R' => self.direction = (self.direction + 1) % 4,
                'L' => self.direction = (self.direction + 3) % 4,
                _ => unreachable!(),
            },
        }
    }
}

fn parse(str: &str) -> (Map, Vec<Instruction>) {
    let (coords, instructions) = str.split_once("\n\n").unwrap();
    let coords = coords
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let start = coords[0].iter().position(|c| *c == '.').unwrap();
    let instr_gex = Regex::new(r"(\d+)|([LR])").unwrap();
    let instructions = instr_gex
        .captures_iter(instructions)
        .map(|c| c.get(0).unwrap().as_str())
        .map(|i| match i.parse::<usize>() {
            Ok(v) => Instruction::Move(v),
            Err(_) => Instruction::Turn(i.chars().into_iter().nth(0).unwrap()),
        })
        .collect_vec();
    (
        Map {
            location: (0, start),
            direction: 0,
            coords,
        },
        instructions,
    )
}

pub fn solve(str: &str) -> (usize, usize) {
    let (mut map, instructions) = parse(str);
    let mut c_map = map.clone();
    instructions.iter().for_each(|i| c_map.eval_instruction(i));
    let s1 = 1000 * (c_map.location.0 + 1) + 4 * (c_map.location.1 + 1) + c_map.direction;
    instructions.iter().for_each(|i| map.eval_instruction_cube(i));
    let s2 = 1000 * (map.location.0 + 1) + 4 * (map.location.1 + 1) + map.direction;
    (s1, s2)
}
