mod util;
mod day2;

fn main() {
    let input = util::get_input(2022, 2);
    println!("{:?}", day2::solve(&input));
}