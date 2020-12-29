use eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn init_tracing(level: &str) {
    color_eyre::install().unwrap();

    let fmt_layer = fmt::layer().with_target(false);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(level))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

#[cfg(test)]
use ctor::ctor;

#[ctor]
#[cfg(test)]
fn init_test() {
    init_tracing("debug");
}

fn main() -> Result<()> {
    init_tracing("info");

    day20::run()?;
    day19::run()?;
    day18::run()?;
    day17::run()?;
    day16::run()?;
    day15::run()?;
    day14::run()?;
    day13::run()?;
    day12::run()?;
    day11::run()?;
    day10::run()?;
    day9::run();
    day8::run();
    day7::run();
    day6::run();
    day5::run();
    day4::run();
    day3::run();
    day2::run();
    day1::run();

    Ok(())
}
