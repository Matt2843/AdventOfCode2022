mod util;
mod day1;
fn main() {
    let input = util::get_input(1);
    println!("{:?}", day1::solve(&input));
}
