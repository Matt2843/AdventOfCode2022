mod util;
mod day13;

fn main() {
    let input = util::get_input(2022, 13, false);
    println!("{:?}", day13::solve(&input));
}