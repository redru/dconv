use std::{fmt::Debug, str::FromStr};

use anyhow::{Result, anyhow};
use chrono::{DateTime, Local, TimeZone, offset::FixedOffset};
use clap::Parser;

use crate::input_types::{InputDateType, Operation};

mod input_types;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "dconv")]
struct Args {
    #[arg(index = 1)]
    date: String,

    #[arg(index = 2, allow_hyphen_values = true)]
    operation: Option<Operation>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let result = convert(&args.date, args.operation.as_ref())?;
    println!("{}", result);

    Ok(())
}

/// Formats a DateTime in both UTC and with the specified offset
fn format_datetime<T: TimeZone>(date_time: &DateTime<T>, local_offset: &FixedOffset) -> String {
    format!(
        "{}\n{}",
        date_time.to_rfc3339(),
        date_time.with_timezone(local_offset).to_rfc3339()
    )
}

/// Converts a DateTime to a timestamp in milliseconds
fn datetime_to_timestamp<T: TimeZone>(date_time: &DateTime<T>) -> String {
    date_time.timestamp_millis().to_string()
}

fn convert(input: &str, operation: Option<&Operation>) -> Result<String> {
    // Hack to a reference to the local Offset without using the module TZ
    let local_date_time = Local::now();
    let local_offset = local_date_time.offset();

    let input_date_type = InputDateType::from_str(input).map_err(|e| anyhow!(e))?;

    let mut date_time = match input_date_type {
        InputDateType::DateTime(date_time)
        | InputDateType::Timestamp(date_time)
        | InputDateType::Now(date_time) => date_time,
    };

    if let Some(operation) = operation {
        match operation {
            Operation::Sum(duration) => {
                date_time += *duration;
            }
            Operation::Subtract(duration) => {
                date_time -= *duration;
            }
        }
    }

    Ok([
        format_datetime(&date_time, local_offset),
        datetime_to_timestamp(&date_time),
    ]
    .join("\n"))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    // Test conversion from RFC3339 to timestamp
    #[test]
    fn test_convert_rfc3339() {
        let result = convert("2025-06-24T20:18:00Z", None).expect("Failed to convert str");
        // The result should contain the RFC3339 date and the timestamp
        assert!(result.contains("2025-06-24T20:18:00"));
        assert!(result.contains("1750796280000"));
    }

    // Test conversion from seconds timestamp (10 digits) to RFC3339
    #[test]
    fn test_convert_seconds_timestamp() {
        let timestamp = "1750796280"; // 2025-06-24T20:18:00Z in seconds
        let result = convert(timestamp, None).expect("Failed to convert timestamp");

        // Check the first line contains the UTC time
        assert!(result.contains("2025-06-24T20:18:00"));
    }

    // Test conversion from milliseconds timestamp (13 digits) to RFC3339
    #[test]
    fn test_convert_millis_timestamp() {
        let timestamp = "1750796280000"; // 2025-06-24T20:18:00Z in milliseconds
        let result = convert(timestamp, None).expect("Failed to convert timestamp");

        // Check the first line contains the UTC time
        assert!(result.contains("2025-06-24T20:18:00"));
    }

    // Test conversion from microseconds timestamp (16 digits) to RFC3339
    #[test]
    fn test_convert_micros_timestamp() {
        let timestamp = "1750796280000000"; // 2025-06-24T20:18:00Z in microseconds
        let result = convert(timestamp, None).expect("Failed to convert timestamp");

        // Check the first line contains the UTC time
        assert!(result.contains("2025-06-24T20:18:00"));
    }

    // Test conversion from nanoseconds timestamp (19 digits) to RFC3339
    #[test]
    fn test_convert_nanos_timestamp() {
        let timestamp = "1750796280000000000"; // 2025-06-24T20:18:00Z in nanoseconds
        let result = convert(timestamp, None).expect("Failed to convert timestamp");

        // Check the first line contains the UTC time
        assert!(result.contains("2025-06-24T20:18:00"));
    }

    // Test error case for unsupported timestamp length
    #[test]
    fn test_unsupported_timestamp_length() {
        let timestamp = "17507962"; // Too few digits
        let result = convert(timestamp, None);
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Timestamp of size 8 is not supported"));
    }

    // Test invalid RFC3339 string
    #[test]
    fn test_invalid_rfc3339() {
        let invalid_date = "2025-13-24T20:18:00Z"; // Invalid month
        let result = convert(invalid_date, None);
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Unsupported date value '2025-13-24T20:18:00Z'"));
    }

    // Test round trip conversion
    #[test]
    fn test_round_trip_conversion() {
        // Start with a known date
        let original_date = "2025-06-24T20:18:00Z";

        // Convert to timestamp
        let timestamp_result =
            convert(original_date, None).expect("Failed to convert to timestamp");
        // Extract the timestamp line (last line)
        let timestamp = timestamp_result
            .lines()
            .last()
            .expect("No timestamp line in convert result");

        // Convert back to date (will include both UTC and local time)
        let date_result = convert(timestamp, None).expect("Failed to convert back to date");

        // The result should contain the original date in RFC3339 format
        assert!(date_result.contains("2025-06-24T20:18:00"));
    }

    // Test Operation::Sum with days
    #[test]
    fn test_operation_sum_days() {
        let input_date = "2025-06-24T20:18:00Z";
        let operation = Some(Operation::from_str("+2d").unwrap());
        let result =
            convert(input_date, operation.as_ref()).expect("Failed to apply sum operation");
        // Expect 2025-06-26T20:18:00Z
        assert!(result.contains("2025-06-26T20:18:00"));
    }

    // Test Operation::Subtract with hours
    #[test]
    fn test_operation_subtract_hours() {
        let input_date = "2025-06-24T20:18:00Z";
        let operation = Some(Operation::from_str("-3h").unwrap());
        let result =
            convert(input_date, operation.as_ref()).expect("Failed to apply subtract operation");
        // Expect 2025-06-24T17:18:00Z
        assert!(result.contains("2025-06-24T17:18:00"));
    }

    // Test Operation::Sum with minutes
    #[test]
    fn test_operation_sum_minutes() {
        let input_date = "2025-06-24T20:18:00Z";
        let operation = Some(Operation::from_str("+45m").unwrap());
        let result =
            convert(input_date, operation.as_ref()).expect("Failed to apply sum operation");
        // Expect 2025-06-24T21:03:00Z
        assert!(result.contains("2025-06-24T21:03:00"));
    }

    // Test invalid operation string
    #[test]
    fn test_invalid_operation_string() {
        let operation = Operation::from_str("++1x");
        assert!(operation.is_err());
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
