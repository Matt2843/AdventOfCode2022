pub fn solve(str: &str) -> (usize, usize) {
    let (s1, _) = str
        .lines()
        .into_iter()
        .fold((0, Vec::<usize>::new()),
        |(mut acc_size, mut stack), line| {
            if line == "$ cd .." {
                let dir_size = stack.pop().expect("empty stack!?");
                if dir_size <= 100_000 {
                    acc_size += dir_size;
                }
            } else if line.starts_with("$ cd") {
                stack.push(0);
            } else if line.starts_with(|c: char| c.is_numeric()) {
                let file_size: usize = line.split_ascii_whitespace().next().unwrap().parse().unwrap();
                stack.iter_mut().for_each(|x| *x += file_size);
            }
            (acc_size, stack)
        }
    );
    let (mut all, _) = str.lines()
        .into_iter()
        .fold((Vec::<usize>::new(), Vec::<usize>::new()), 
            |(mut all, mut stack), line| {
                if line == "$ cd .." {
                    all.push(stack.pop().expect("empty stack!?"));
                } else if line.starts_with("$ cd") {
                    stack.push(0);
                } else if line.starts_with(|c: char| c.is_numeric()) {
                    let file_size: usize = line.split_ascii_whitespace().nth(0).unwrap().parse().unwrap();
                    stack.iter_mut().for_each(|x| *x += file_size);
                }
                (all, stack)
            });
    all.sort();
    let avail = 70000000 - all.last().unwrap();
    let needed = 40000000 - avail;
    let smallest = all.iter().rev().find(|x| **x <= needed);
    (s1, *smallest.unwrap())
}
