mod util;
mod day3;

fn main() {
    let input = util::get_input(2022, 3);
    println!("{:?}", day3::solve(&input));
}