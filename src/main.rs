use std::{io::BufReader, fs::File};

use clap::Parser;

mod day_one;
mod day_two;
mod day_three;

#[derive(Parser)]
#[command(version = "0.0.1", author = "Evan Merlock")]
struct AdventOfCodeOptions {
    #[clap(short, long)]
    in_file: String,
    #[clap(short, long)]
    day: u32,
    #[clap(short, long)]
    question: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = AdventOfCodeOptions::parse();

    let in_file: BufReader<File> = BufReader::new(File::open(options.in_file)?);

    match (options.day, options.question) {
        (1, 1) => day_one::question_one(in_file)?,
        (1, 2) => day_one::question_two(in_file)?,
        (2, 1) => day_two::question_one(in_file)?,
        (2, 2) => day_two::question_two(in_file)?,
        (3, 1) => day_three::question_one(in_file)?,
        (3, 2) => day_three::question_two(in_file)?,
        _ => panic!("Invalid day/question combination!")
    };

    Ok(())

}