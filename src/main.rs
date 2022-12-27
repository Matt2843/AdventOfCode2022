mod util;
mod day24;
fn main() {
    let input = util::get_input(2022, 24, true);
    println!("{:?}", day24::solve(&input));
}