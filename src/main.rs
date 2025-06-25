use std::fmt::Debug;

use anyhow::{Context, Result, bail};
use chrono::{DateTime, Local};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "dconv")]
struct Args {
    #[arg(index = 1)]
    date: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let result = convert(&args.date)?;
    println!("{}", result);

    Ok(())
}

fn convert(input: &str) -> Result<String> {
    if let Ok(timestamp) = input.parse::<i64>() {
        let date_time = match input.len() {
            10 => DateTime::from_timestamp(timestamp, 0),
            13 => DateTime::from_timestamp_millis(timestamp),
            16 => DateTime::from_timestamp_micros(timestamp),
            19 => Some(DateTime::from_timestamp_nanos(timestamp)),
            _ => bail!("Timestamp of size {} is not supported", input.len()),
        }
        .context("Failed to parse timestamp")?;

        Ok(format!(
            "{}\n{}",
            date_time,
            date_time.with_timezone(Local::now().offset())
        ))
    } else {
        let date_time =
            DateTime::parse_from_rfc3339(input).context("Error parsing rfc3339 string")?;

        Ok(date_time.timestamp_millis().to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_convert_rfc3339() {
        assert_eq!(
            convert(&"2025-06-24T20:18:00Z").expect("Failed to convert str"),
            String::from_str("1750796280000").expect("Failed to parse str")
        );
    }
}
