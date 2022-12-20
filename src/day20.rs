use std::collections::VecDeque;

fn parse(str: &str, dec_key: i64) -> VecDeque<(usize, i64)> {
    str.trim().lines().enumerate()
        .map(|(u, l)| (u, l.trim().parse::<i64>().unwrap() * dec_key))
        .fold(VecDeque::new(), |mut acc, x| {
            acc.push_back(x);
            acc
        })
}

fn get_result(deq: &VecDeque<(usize, i64)>) -> i64 {
    let idx = deq.iter().enumerate().find(|(_, (_, val))| *val == 0).unwrap().0;
    [idx+1000,idx+2000,idx+3000].iter().map(|i| deq[i%deq.len()].1).sum()
}

fn slv(deq: &mut VecDeque<(usize, i64)>, times: usize) -> i64 {
    for _ in 0..times {
        for oi in 0..deq.len() {
            let shift_len = deq.iter().enumerate().find(|(_,(i,_))| oi == *i).unwrap().0;
            deq.rotate_left(shift_len);
            let front_val = deq.pop_front().unwrap();
            deq.rotate_left(front_val.1.rem_euclid((deq.len()) as i64) as usize);
            deq.push_back(front_val);
        }
    }
    get_result(deq)
}

pub fn solve(str: &str) -> (usize, usize) {
    let s1 = slv(&mut parse(str, 1), 1);
    let s2 = slv(&mut parse(str, 811589153), 10);
    (s1 as usize, s2 as usize)
}