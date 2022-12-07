mod util;
mod day7;

fn main() {
    let input = util::get_input(2022, 7);
    println!("{:?}", day7::solve(&input));
}