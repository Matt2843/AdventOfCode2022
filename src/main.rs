mod util;
mod day19;
fn main() {
    let input = util::get_input(2022, 19, false);
    println!("{:?}", day19::solve(&input));
}