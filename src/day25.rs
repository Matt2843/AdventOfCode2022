fn snafu_to_decimal(snafu_str: &str) -> isize {
    snafu_str.chars().rev().enumerate()
        .map(|(exp, ch)| {
            match ch {
                '=' => -2 * 5isize.pow(exp as u32),
                '-' => -1 * 5isize.pow(exp as u32),
                '0' => 0,
                '1' => 1 * 5isize.pow(exp as u32),
                '2' => 2 * 5isize.pow(exp as u32),
                _ => panic!("{:?}", (exp, ch))
            }
        }).sum()
}

fn decimal_to_snafu(mut decimal: isize) -> String {
    let mut snafu = String::new();
    while decimal > 0 {
        let rem = decimal % 5;
        decimal = decimal / 5 + if rem > 2 { 1 } else { 0 };
        snafu.push(match rem {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!("{:?}", (rem, decimal))
        })
    }
    if snafu.is_empty() {
        snafu.push('0')
    }
    snafu.chars().rev().collect()
}

pub fn solve(str: &str) -> (String, usize) {
    let snafu_sum: isize = str.trim().lines().map(|l| snafu_to_decimal(l)).sum();
    (decimal_to_snafu(snafu_sum), 0)
}

#[cfg(test)]
mod tests {
    use super::{snafu_to_decimal, decimal_to_snafu};
    #[test]
    fn snafu_to_decimal_test() {
        assert_eq!(1, snafu_to_decimal("1"));
        assert_eq!(3, snafu_to_decimal("1="));
        assert_eq!(4, snafu_to_decimal("1-"));
        assert_eq!(5, snafu_to_decimal("10"));
        assert_eq!(9, snafu_to_decimal("2-"));
        assert_eq!(20, snafu_to_decimal("1-0"));
        assert_eq!(12345, snafu_to_decimal("1-0---0"));
        assert_eq!(314159265, snafu_to_decimal("1121-1110-1=0"));
    }

    #[test]
    fn decimal_to_snafu_test() {
        assert_eq!("1=-0-2".to_string(), decimal_to_snafu(1747));
        assert_eq!("12111".to_string(), decimal_to_snafu(906));
        assert_eq!("2=0=".to_string(), decimal_to_snafu(198));
        assert_eq!("21".to_string(), decimal_to_snafu(11));
        assert_eq!("2=01".to_string(), decimal_to_snafu(201));
        assert_eq!("111".to_string(), decimal_to_snafu(31));
        assert_eq!("20012".to_string(), decimal_to_snafu(1257));
        assert_eq!("112".to_string(), decimal_to_snafu(32));
    }
}