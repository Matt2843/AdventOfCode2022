mod util;
mod day21;
fn main() {
    let input = util::get_input(2022, 21, false);
    println!("{:?}", day21::solve(&input));
}