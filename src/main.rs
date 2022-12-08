mod util;
mod day8;

fn main() {
    let input = util::get_input(2022, 8);
    println!("{:?}", day8::solve(&input));
}