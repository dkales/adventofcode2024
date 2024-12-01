use std::path::PathBuf;

use aoc_traits::AdventOfCodeSolutions;
use clap::Parser;
use color_eyre::Result;
use secrecy::SecretString;

#[derive(Parser)]
struct AoCRunner {
    #[clap(short, long)]
    day: usize,
    #[clap(short, long)]
    input: PathBuf,
    #[clap(short, long, env = "AGE_PASSPHRASE")]
    passphrase: SecretString,
}

fn main() -> Result<()> {
    let args = AoCRunner::parse();

    let identity = age::scrypt::Identity::new(args.passphrase);
    let enc_input = std::fs::read(&args.input)?;
    let input = String::from_utf8(age::decrypt(&identity, &enc_input)?)?;

    meta::AoC2024::solve_day(args.day, &input).map_err(|e| color_eyre::eyre::eyre!(e))?;

    Ok(())
}
