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
    passphrase: Option<SecretString>,
}

fn main() -> Result<()> {
    let args = AoCRunner::parse();

    let input = if args.input.extension().map(|e| e == "age").unwrap_or(false) {
        let age_passphrase = args
            .passphrase
            .ok_or_else(|| color_eyre::eyre::eyre!("Passphrase is required for encrypted input"))?;
        let identity = age::scrypt::Identity::new(age_passphrase);
        let enc_input = std::fs::read(&args.input)?;
        String::from_utf8(age::decrypt(&identity, &enc_input)?)?
    } else {
        std::fs::read_to_string(&args.input)?
    };

    meta::AoC2024::solve_day(args.day, &input).map_err(|e| color_eyre::eyre::eyre!(e))?;

    Ok(())
}
