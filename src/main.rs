mod util;
mod day5;

fn main() {
    let input = util::get_input(2022, 5);
    println!("{:?}", day5::solve(&input));
}