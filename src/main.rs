mod util;
mod day12;

fn main() {
    let input = util::get_input(2022, 12);
    println!("{:?}", day12::solve(&input));
}