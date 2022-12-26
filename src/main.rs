mod util;
mod day22;
fn main() {
    let input = util::get_input(2022, 22, false);
    println!("{:?}", day22::solve(&input));
}