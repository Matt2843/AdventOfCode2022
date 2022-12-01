mod util;
mod day1;
mod day2;

fn main() {
    let input = util::get_input(2022, 1);
    println!("{:?}", day1::solve(&input));
}
