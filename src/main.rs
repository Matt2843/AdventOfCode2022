mod util;
mod day14;

fn main() {
    let input = util::get_input(2022, 14, false);
    println!("{:?}", day14::solve(&input));
}