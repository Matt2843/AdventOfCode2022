mod util;
mod day18;

fn main() {
    let input = util::get_input(2022, 18, false);
    println!("{:?}", day18::solve(&input));
}