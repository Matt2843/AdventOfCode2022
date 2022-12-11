mod util;
mod day11;

fn main() {
    let input = util::get_input(2022, 11);
    println!("{:?}", day11::solve(&input));
}