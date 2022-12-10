mod util;
mod day10;

fn main() {
    let input = util::get_input(2022, 10);
    println!("{:?}", day10::solve(&input));
}