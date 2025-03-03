#![feature(hash_set_entry)]
#![feature(try_blocks)]
#![feature(iter_intersperse)]

pub mod maze;
mod program;
mod utls;
mod year_2019;

fn main() {
    run_year("2019");

    println!("Hello, world!");
}
fn run_year(year: &str) {
    match year {
        "2019" => year_2019::run(),
        _ => unreachable!("year not implemented"),
    }
}
