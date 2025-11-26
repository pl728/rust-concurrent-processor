mod part1;
mod part2a;
mod part2b;

fn main() {
    // println!("===Part 1: Basic Threads===");
    // part1::run();

    // println!("===Part 2a: Message Passing (naive)===");
    // part2a::run();

    println!("===Part 2a: Message Passing (thread pool)===");
    part2b::run();
}