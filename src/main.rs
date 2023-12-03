use tracing_subscriber::{fmt, filter::LevelFilter};

mod day1;
mod day2;
mod day3;
mod utils;

fn main() {
    let format = fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(false)
        .compact();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .event_format(format)
        .init();

    println!("Executing day 1 problems...");
    println!("  p1 = {}", day1::pt_1());
    println!("\nExecuting day 2 problems...");
    println!("  p1 = {}", day2::pt_1());
    println!("  p2 = {}", day2::pt_2());
    println!("\nExecuting day 3 problems...");
    // println!("  e1 = {}", day3::ex_1());
    // println!("  p1 = {}", day3::pt_1());
    println!("  e2 = {}", day3::ex_2());
    println!("  p2 = {}", day3::pt_2());
}
