# dconv

**dconv** is a lightweight command-line utility that converts between Unix timestamps and RFC 3339 date-time strings. It detects the input format automatically and converts it to the other, making it easy to work with time formats in scripts or manually from the terminal.

---

## 🧭 Features

- ✅ Convert Unix timestamps (e.g., `1720000000`) to RFC 3339 date-time strings (e.g., `2024-07-03T10:13:20Z`)
- ✅ Parse RFC 3339 strings and convert them back to Unix timestamps
- ✅ No flags or options needed — usage is minimal and intuitive
- ✅ Input format is auto-detected
- ✅ Fast and dependency-free (ideal for scripting)

---

## ⚙️ Usage

```
dconv <timestamp | rfc3339>
```

Where:

- `<timestamp>` is a Unix timestamp (seconds since epoch, e.g., `1720000000`)
- `<rfc3339>` is a valid RFC 3339 date-time string (e.g., `2024-07-03T10:13:20Z`)

---

## 📌 Examples

Convert a Unix timestamp to RFC 3339:

```
$ dconv 1720000000
2024-07-03T10:13:20Z
```

Convert an RFC 3339 string to a Unix timestamp:

```
$ dconv 2024-07-03T10:13:20Z
1720000000
```

---

## 📥 Installation

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

- All conversions are done using UTC time.
- Input RFC 3339 strings must strictly follow the format:
  `YYYY-MM-DDTHH:MM:SSZ`
  Example: `2024-07-03T10:13:20Z`
- The tool only supports seconds-level precision.

---

## 🪪 License

This project is licensed under the **MIT License**. See the `LICENSE` file for more details.

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
