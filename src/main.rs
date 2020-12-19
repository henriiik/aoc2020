use eyre::Result;
use tracing::Level;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn init_tracing(level: Level) -> Result<()> {
    let coll = tracing_subscriber::fmt().with_max_level(level).finish();
    Ok(tracing::subscriber::set_global_default(coll)?)
}

fn main() -> Result<()> {
    init_tracing(Level::INFO)?;

    day1::run();
    day2::run();
    day3::run();
    day4::run();
    day5::run();
    day6::run();
    day7::run();
    day8::run();
    day9::run();

    Ok(())
}
