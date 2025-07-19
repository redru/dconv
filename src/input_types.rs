use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(Unit::Seconds),
            "m" => Ok(Unit::Minutes),
            "h" => Ok(Unit::Hours),
            "d" => Ok(Unit::Days),
            _ => Err(format!("Unsupported unit {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Plus,
    Minus,
}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Symbol::Plus),
            "-" => Ok(Symbol::Minus),
            _ => Err(format!("Unrecognized symbol {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Sum(Duration),
    Subtract(Duration),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            return Err(format!("Invalid operation {}", s));
        }

        let (symbol, value, unit) = split_operation_input(s)?;

        let duration = match unit {
            Unit::Seconds => Duration::seconds(value),
            Unit::Minutes => Duration::minutes(value),
            Unit::Hours => Duration::hours(value),
            Unit::Days => Duration::days(value),
        };

        match symbol {
            Symbol::Plus => Ok(Operation::Sum(duration)),
            Symbol::Minus => Ok(Operation::Subtract(duration)),
        }
    }
}

/// Splits an operation string into (Symbol, i64, Unit).
/// Example: "+10m" -> (Symbol::Plus, 10, Unit::Minutes)
fn split_operation_input(input: &str) -> Result<(Symbol, i64, Unit), String> {
    if input.len() < 3 {
        return Err(format!("Input too short: {}", input));
    }

    let symbol_str = &input[0..1];
    let unit_str = &input[input.len() - 1..];
    let value_str = &input[1..input.len() - 1];

    let symbol = Symbol::from_str(symbol_str)?;
    let value = value_str
        .parse::<i64>()
        .map_err(|e| format!("Invalid value '{}': {}", value_str, e))?;
    let unit = Unit::from_str(unit_str)?;

    Ok((symbol, value, unit))
}

#[derive(Debug, Clone, Copy)]
pub enum InputDateType {
    DateTime(DateTime<Utc>),
    Timestamp(DateTime<Utc>),
    Now(DateTime<Utc>),
}

impl FromStr for InputDateType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "now" {
            Ok(InputDateType::Now(Utc::now()))
        } else if let Ok(timestamp) = s.parse::<i64>() {
            let date_time = match s.len() {
                10 => DateTime::from_timestamp(timestamp, 0),
                13 => DateTime::from_timestamp_millis(timestamp),
                16 => DateTime::from_timestamp_micros(timestamp),
                19 => Some(DateTime::from_timestamp_nanos(timestamp)),
                _ => None,
            }
            .ok_or_else(|| format!("Timestamp of size {} is not supported", s.len()))?;

            Ok(InputDateType::Timestamp(date_time))
        } else if let Ok(date_time) = DateTime::parse_from_rfc3339(s) {
            Ok(InputDateType::DateTime(date_time.to_utc()))
        } else {
            Err(format!("Unsupported date value '{}'", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_unit_from_str() {
        assert_eq!(Unit::from_str("s").unwrap(), Unit::Seconds);
        assert_eq!(Unit::from_str("m").unwrap(), Unit::Minutes);
        assert_eq!(Unit::from_str("h").unwrap(), Unit::Hours);
        assert_eq!(Unit::from_str("d").unwrap(), Unit::Days);
        assert!(Unit::from_str("x").is_err());
    }

    #[test]
    fn test_symbol_from_str() {
        assert_eq!(Symbol::from_str("+").unwrap(), Symbol::Plus);
        assert_eq!(Symbol::from_str("-").unwrap(), Symbol::Minus);
        assert!(Symbol::from_str("*").is_err());
    }

    #[test]
    fn test_operation_from_str() {
        match Operation::from_str("+10m").unwrap() {
            Operation::Sum(d) => assert_eq!(d, Duration::minutes(10)),
            _ => panic!("Expected Sum"),
        }
        match Operation::from_str("-5h").unwrap() {
            Operation::Subtract(d) => assert_eq!(d, Duration::hours(5)),
            _ => panic!("Expected Subtract"),
        }
        assert!(Operation::from_str("10m").is_err());
        assert!(Operation::from_str("+xx").is_err());
    }

    #[test]
    fn test_input_date_type_from_str() {
        // RFC3339
        let dt = InputDateType::from_str("2025-06-24T20:18:00Z").unwrap();
        match dt {
            InputDateType::DateTime(dt) => assert_eq!(dt.to_rfc3339(), "2025-06-24T20:18:00+00:00"),
            _ => panic!("Expected DateTime"),
        }
        // Timestamp (seconds)
        let ts = InputDateType::from_str("1750796280").unwrap();
        match ts {
            InputDateType::Timestamp(dt) => {
                assert_eq!(dt.to_rfc3339(), "2025-06-24T20:18:00+00:00")
            }
            _ => panic!("Expected Timestamp"),
        }
        // Now
        let now = InputDateType::from_str("now").unwrap();
        match now {
            InputDateType::Now(_) => {} // Accept any value
            _ => panic!("Expected Now"),
        }
        // Invalid
        assert!(InputDateType::from_str("notadate").is_err());
    }
}
