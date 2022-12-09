mod util;
mod day9;

fn main() {
    let input = util::get_input(2022, 9);
    println!("{:?}", day9::solve(&input));
}