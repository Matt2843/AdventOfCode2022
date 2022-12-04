mod util;
mod day4;

fn main() {
    let input = util::get_input(2022, 4);
    println!("{:?}", day4::solve(&input));
}