# dconv

[![Tests](https://github.com/redru/dconv/actions/workflows/tests.yml/badge.svg)](https://github.com/redru/dconv/actions/workflows/tests.yml)

**dconv** is a lightweight command-line utility that converts between timestamps and RFC 3339 date-time strings. It detects the input format automatically and performs bidirectional conversions, making it easy to work with different time formats in scripts or manually from the terminal.

---

## 🧭 Features

- ✅ Convert Unix timestamps to RFC 3339 date-time strings
- ✅ Convert RFC 3339 date-time strings to Unix timestamps
- ✅ Perform operations (+, -, /, *) on timestamps and date-time strings
- ✅ Support for multiple timestamp precisions:
  - Seconds (10 digits, e.g., `1750796280`)
  - Milliseconds (13 digits, e.g., `1750796280000`)
  - Microseconds (16 digits, e.g., `1750796280000000`)
  - Nanoseconds (19 digits, e.g., `1750796280000000000`)
- ✅ Parse RFC 3339 strings and convert them to millisecond timestamps
- ✅ Output is always 3 lines: ISO 8601 UTC, ISO 8601 in your local offset, and the Unix timestamp (in ms)
- ✅ No flags or options needed — usage is minimal and intuitive
- ✅ Input format is auto-detected

---

## ⚙️ Usage

```
dconv <timestamp | rfc3339 | now>
```

Where:

- `<timestamp>` is a Unix timestamp in seconds (10 digits), milliseconds (13 digits), microseconds (16 digits), or nanoseconds (19 digits)
- `<rfc3339>` is a valid RFC 3339 date-time string (e.g., `2025-06-24T20:18:00Z`)
- `now` can be used as the first parameter to represent the current datetime

---

## 📌 Examples

### Convert a timestamp to RFC 3339

```
$ dconv 1750796280
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00
1750796280000
```

The output shows:
- First line: Time in UTC (ISO 8601)
- Second line: Time in your local timezone (ISO 8601)
- Third line: Unix timestamp in milliseconds

### Convert timestamps with different precisions

```
$ dconv 1750796280000        # Milliseconds
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00
1750796280000
$ dconv 1750796280000000     # Microseconds
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00
1750796280000
$ dconv 1750796280000000000  # Nanoseconds
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00
1750796280000
```

### Convert an RFC 3339 string to a timestamp

```
$ dconv 2025-06-24T20:18:00Z
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00
1750796280000
```

RFC 3339 strings are always converted to millisecond precision timestamps.

### Use `now` as the current datetime

You can use `now` as the first parameter to represent the current datetime:

```
$ dconv now
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00
1750796280000
```

You can also use it with operations:

```
$ dconv now +1d
2025-06-25T20:18:00+00:00
2025-06-25T22:18:00+02:00
1750882680000
```

### Perform operations on timestamps and date-time strings

You can use the following units for operations:

- `s` = seconds
- `m` = minutes
- `h` = hours
- `d` = days

For example: `+10m`, `-2h`, `+1d`

#### Add days to a date-time

```
$ dconv 2025-06-24T20:18:00Z +2d
2025-06-26T20:18:00+00:00
2025-06-26T22:18:00+02:00
1750969080000
```

#### Subtract hours from a date-time

```
$ dconv 2025-06-24T20:18:00Z -3h
2025-06-24T17:18:00+00:00
2025-06-24T19:18:00+02:00
1750785480000
```

#### Add minutes to a timestamp

```
$ dconv 1750796280 +45m
2025-06-24T21:03:00+00:00
2025-06-24T23:03:00+02:00
1750798980000
```

---

## 📥 Installation

You can install **dconv** using [Homebrew](https://brew.sh):

```
brew install redru/dconv/dconv
```

---

## 📖 Notes

- When converting timestamps to dates, both UTC and local timezone representations are displayed.
- When converting RFC 3339 strings to timestamps, timestamps are returned in milliseconds.
- Input RFC 3339 strings must follow the RFC 3339 format (e.g., `2025-06-24T20:18:00Z` or `2025-06-24T22:18:00+02:00`).
- Timestamps must be exactly 10, 13, 16, or 19 digits long, representing seconds, milliseconds, microseconds, or nanoseconds respectively.

---

## 🪪 License

This project is licensed under the **Apache-2.0 License**. See the `LICENSE` file for more details.

---

## 🤝 Contributing

Contributions are welcome!

If you:

- Found a bug
- Want a new feature
- Have a question or suggestion

Feel free to open an issue or submit a pull request.

---

## 🧑‍💻 Author

**dconv** was created and is maintained by **redru**.

GitHub: `https://github.com/redru/dconv`
