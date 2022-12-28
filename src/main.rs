mod util;
mod day25;
fn main() {
    let input = util::get_input(2022, 25, false);
    println!("{:?}", day25::solve(&input));
}