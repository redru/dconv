# dconv

[![Rust Tests](https://github.com/redru/dconv/actions/workflows/rust-tests.yml/badge.svg)](https://github.com/redru/dconv/actions/workflows/rust-tests.yml)

**dconv** is a lightweight command-line utility that converts between timestamps and RFC 3339 date-time strings. It detects the input format automatically and performs bidirectional conversions, making it easy to work with different time formats in scripts or manually from the terminal.

---

## 🧭 Features

- ✅ Convert Unix timestamps to RFC 3339 date-time strings
- ✅ Support for multiple timestamp precisions:
  - Seconds (10 digits, e.g., `1750796280`)
  - Milliseconds (13 digits, e.g., `1750796280000`)
  - Microseconds (16 digits, e.g., `1750796280000000`)
  - Nanoseconds (19 digits, e.g., `1750796280000000000`)
- ✅ Parse RFC 3339 strings and convert them to millisecond timestamps
- ✅ Display both UTC and local timezone representations
- ✅ No flags or options needed — usage is minimal and intuitive
- ✅ Input format is auto-detected

---

## ⚙️ Usage

```
dconv <timestamp | rfc3339>
```

Where:

- `<timestamp>` is a Unix timestamp in seconds (10 digits), milliseconds (13 digits), microseconds (16 digits), or nanoseconds (19 digits)
- `<rfc3339>` is a valid RFC 3339 date-time string (e.g., `2025-06-24T20:18:00Z`)

---

## 📌 Examples

### Convert a timestamp to RFC 3339

```
$ dconv 1750796280
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00
```

The output shows:
- First line: Time in UTC timezone
- Second line: Time in your local timezone

### Convert timestamps with different precisions

```
$ dconv 1750796280000        # Milliseconds
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00

$ dconv 1750796280000000     # Microseconds
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00

$ dconv 1750796280000000000  # Nanoseconds
2025-06-24T20:18:00+00:00
2025-06-24T22:18:00+02:00
```

### Convert an RFC 3339 string to a timestamp

```
$ dconv 2025-06-24T20:18:00Z
1750796280000
```

RFC 3339 strings are always converted to millisecond precision timestamps.

---

## 📥 Installation

### From Source

1. Clone the repository
2. Build with Cargo:

```
cargo build --release
```

3. Move the binary to a location in your `$PATH`:

```
mv target/release/dconv /usr/local/bin/
```

### Manual Installation

1. Download the `dconv` binary for your platform.
2. Make it executable:

```
chmod +x dconv
```

3. Move it to a location in your `$PATH`, for example:

```
mv dconv /usr/local/bin/
```

4. Now you can use `dconv` from anywhere in your terminal.

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
