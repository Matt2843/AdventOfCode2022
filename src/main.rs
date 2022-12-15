mod util;
mod day15;

fn main() {
    let input = util::get_input(2022, 15, false);
    println!("{:?}", day15::solve(&input));
}