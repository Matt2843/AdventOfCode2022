mod util;
mod day6;

fn main() {
    let input = util::get_input(2022, 6);
    println!("{:?}", day6::solve(&input));
}