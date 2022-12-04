pub fn solve(str: &str) -> (usize, usize) {
    // probably faster to complete s2 if:
    // in s1 compute the sets and check set-difference both ways is Ø in filter
    // only change in s2 would then be to check if intersection != Ø
    let parsed = str.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(r1,r2)| (r1.split_once('-').unwrap(), r2.split_once('-').unwrap()))
        .map(|((x1,y1),(x2,y2))| ((x1.parse::<usize>().unwrap(),y1.parse::<usize>().unwrap()),(x2.parse::<usize>().unwrap(),y2.parse::<usize>().unwrap())));
    let s1 = parsed.clone()
        .filter(|(r1, r2)| (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1))
        .count();
    let s2 = parsed
        .filter(|(r1, r2)| (r1.0 <= r2.0 && r1.1 >= r2.0) || (r2.0 <= r1.0 && r2.1 >= r1.0))
        .count();
    (s1,s2)
}