mod util;
mod day17;

fn main() {
    let input = util::get_input(2022, 17, false);
    println!("{:?}", day17::solve(&input));
}