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

/// Formats a DateTime in both UTC and with the specified offset
fn format_datetime<T: chrono::TimeZone>(
    date_time: &DateTime<T>,
    local_offset: &chrono::offset::FixedOffset,
) -> String {
    format!(
        "{}\n{}",
        date_time.to_rfc3339(),
        date_time.with_timezone(local_offset).to_rfc3339()
    )
}

/// Converts a DateTime to a timestamp in milliseconds
fn datetime_to_timestamp<T: chrono::TimeZone>(date_time: &DateTime<T>) -> String {
    date_time.timestamp_millis().to_string()
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

        // Get the local timezone offset
        Ok(format_datetime(&date_time, &Local::now().offset()))
    } else {
        let date_time =
            DateTime::parse_from_rfc3339(input).context("Error parsing rfc3339 string")?;

        Ok(datetime_to_timestamp(&date_time))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    // Test conversion from RFC3339 to timestamp
    #[test]
    fn test_convert_rfc3339() {
        assert_eq!(
            convert(&"2025-06-24T20:18:00Z").expect("Failed to convert str"),
            String::from_str("1750796280000").expect("Failed to parse str")
        );
    }

    // Test conversion from seconds timestamp (10 digits) to RFC3339
    #[test]
    fn test_convert_seconds_timestamp() {
        let timestamp = "1750796280"; // 2025-06-24T20:18:00Z in seconds
        let result = convert(timestamp).expect("Failed to convert timestamp");

        // Check the first line contains the UTC time
        assert!(result.contains("2025-06-24T20:18:00"));
    }

    // Test conversion from milliseconds timestamp (13 digits) to RFC3339
    #[test]
    fn test_convert_millis_timestamp() {
        let timestamp = "1750796280000"; // 2025-06-24T20:18:00Z in milliseconds
        let result = convert(timestamp).expect("Failed to convert timestamp");

        // Check the first line contains the UTC time
        assert!(result.contains("2025-06-24T20:18:00"));
    }

    // Test conversion from microseconds timestamp (16 digits) to RFC3339
    #[test]
    fn test_convert_micros_timestamp() {
        let timestamp = "1750796280000000"; // 2025-06-24T20:18:00Z in microseconds
        let result = convert(timestamp).expect("Failed to convert timestamp");

        // Check the first line contains the UTC time
        assert!(result.contains("2025-06-24T20:18:00"));
    }

    // Test conversion from nanoseconds timestamp (19 digits) to RFC3339
    #[test]
    fn test_convert_nanos_timestamp() {
        let timestamp = "1750796280000000000"; // 2025-06-24T20:18:00Z in nanoseconds
        let result = convert(timestamp).expect("Failed to convert timestamp");

        // Check the first line contains the UTC time
        assert!(result.contains("2025-06-24T20:18:00"));
    }

    // Test error case for unsupported timestamp length
    #[test]
    fn test_unsupported_timestamp_length() {
        let timestamp = "17507962"; // Too few digits
        let result = convert(timestamp);
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Timestamp of size 8 is not supported"));
    }

    // Test invalid RFC3339 string
    #[test]
    fn test_invalid_rfc3339() {
        let invalid_date = "2025-13-24T20:18:00Z"; // Invalid month
        let result = convert(invalid_date);
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Error parsing rfc3339 string"));
    }

    // Test round trip conversion
    #[test]
    fn test_round_trip_conversion() {
        // Start with a known date
        let original_date = "2025-06-24T20:18:00Z";

        // Convert to timestamp
        let timestamp = convert(original_date).expect("Failed to convert to timestamp");

        // Convert back to date (will include both UTC and local time)
        let date_result = convert(&timestamp).expect("Failed to convert back to date");

        // The result should contain the original date in RFC3339 format
        assert!(date_result.contains("2025-06-24T20:18:00"));
    }

    // Test format_datetime function
    #[test]
    fn test_format_datetime() {
        use chrono::offset::FixedOffset;

        let date_time = DateTime::from_timestamp(1750796280, 0).unwrap();

        // Create a +02:00 offset (2 hours ahead of UTC)
        let offset = FixedOffset::east_opt(2 * 3600).unwrap();

        let formatted = format_datetime(&date_time, &offset);

        // Check that it contains both the UTC and specified timezone representations
        assert!(formatted.contains("2025-06-24T20:18:00"));
        assert!(formatted.contains("2025-06-24T22:18:00+02:00"));
        // Should have two lines
        assert_eq!(formatted.lines().count(), 2);
    }

    // Test datetime_to_timestamp function
    #[test]
    fn test_datetime_to_timestamp() {
        let date_time = DateTime::parse_from_rfc3339("2025-06-24T20:18:00Z").unwrap();
        let timestamp = datetime_to_timestamp(&date_time);

        assert_eq!(timestamp, "1750796280000");
    }
}
