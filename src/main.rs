mod util;
mod day20;
fn main() {
    let input = util::get_input(2022, 20, false);
    println!("{:?}", day20::solve(&input));
}