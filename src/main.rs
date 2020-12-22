use eyre::Result;
use tracing::Level;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn init_tracing(level: Level) {
    let coll = tracing_subscriber::fmt().with_max_level(level).finish();
    tracing::subscriber::set_global_default(coll)
        .expect("should be able to register subscriber once.");
}

#[cfg(test)]
use ctor::ctor;

#[ctor]
#[cfg(test)]
fn init_test() {
    init_tracing(Level::DEBUG);
}

fn main() -> Result<()> {
    init_tracing(Level::INFO);

    day1::run();
    day2::run();
    day3::run();
    day4::run();
    day5::run();
    day6::run();
    day7::run();
    day8::run();
    day9::run();
    day10::run()?;
    day11::run()?;
    day12::run()?;
    day13::run()?;
    day14::run()?;
    day15::run()?;

    Ok(())
}
