pub fn solve(str: &str) -> (usize, usize) {
    let s1 = str.lines()
        .map(|i| i.split_once(" ").unwrap())
        .map(|(e, m)| match e {
            "A" => match m {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3 + 0,
                _ => panic!()
            },
            "B" => match m {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => panic!()
            },
            "C" => match m {
                "X" => 1 + 6,
                "Y" => 2 + 0,
                "Z" => 3 + 3,
                _ => panic!()
            }
            _ => panic!()
        }).sum();

    let s2 = str.lines()
        .map(|i| i.split_once(" ").unwrap())
        .map(|(e, m)| match e {
            "A" => match m {
                "X" => 3 + 0, 
                "Y" => 1 + 3, 
                "Z" => 2 + 6, 
                _ => panic!()
            },
            "B" => match m {
                "X" => 1 + 0, 
                "Y" => 2 + 3, 
                "Z" => 3 + 6, 
                _ => panic!()
            },
            "C" => match m {
                "X" => 2 + 0,
                "Y" => 3 + 3,
                "Z" => 1 + 6,
                _ => panic!()
            }
            _ => panic!()
        }).sum();

    (s1,s2)
}