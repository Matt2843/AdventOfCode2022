mod util;
mod day23;
fn main() {
    let input = util::get_input(2022, 23, false);
    println!("{:?}", day23::solve(&input));
}