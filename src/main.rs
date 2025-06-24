use std::fmt::Debug;

use anyhow::{Context, Result, bail};
use chrono::DateTime;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "dconv")]
struct Args {
    #[arg(index = 1)]
    date: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Ok(timestamp) = args.date.parse::<i64>() {
        let date_time = match args.date.len() {
            10 => DateTime::from_timestamp(timestamp, 0),
            13 => DateTime::from_timestamp_millis(timestamp),
            16 => DateTime::from_timestamp_micros(timestamp),
            19 => Some(DateTime::from_timestamp_nanos(timestamp)),
            _ => bail!("Timestamp of size {} is not supported", args.date.len()),
        }
        .context("Failed to parse timestamp")?;

        println!("{}", date_time.to_rfc3339());
        Ok(())
    } else {
        let date_time =
            DateTime::parse_from_rfc3339(&args.date).context("Error parsing rfc3339 string")?;

        println!("{}", date_time.timestamp_millis());
        Ok(())
    }
}
