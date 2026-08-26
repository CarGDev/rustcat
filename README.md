# rustcat

A Rust reimplementation of the classic Unix `cat` command-line utility. Concatenates files and writes their contents to standard output.

**Zero external dependencies** — built entirely with the Rust standard library.

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/rustcat`.

## Usage

```
rustcat [OPTIONS] [FILE...]
```

If no files are given, or if `-` is passed, reads from standard input.

### Examples

```bash
# Print a file
rustcat file.txt

# Number all lines
rustcat -n file.txt

# Number only non-blank lines, squeeze blank lines, show line endings
rustcat -bsE file.txt

# Show all non-printing characters, tabs, and line endings
rustcat -A file.txt

# Read from stdin
echo "hello" | rustcat -

# Combine multiple flags
rustcat -nEsT file.txt
```

## Flags

| Flag | Long       | Description                                        |
|------|------------|----------------------------------------------------|
| `-b` |            | Number only non-blank output lines (overrides `-n`) |
| `-E` |            | Show `$` at end of each line                       |
| `-n` | `--number` | Number all output lines                            |
| `-s` |            | Squeeze consecutive blank lines into one           |
| `-T` |            | Show tabs as `^I`                                  |
| `-v` |            | Show non-printing characters using `^` and `M-` notation |
| `-A` | `--show-all` | Equivalent to `-vET`                             |
| `-h` | `--help`   | Show help message and exit                         |
| `-`  |            | Read from standard input                           |

Combined short flags are supported (e.g., `-nEsv`).

## Project Structure

```
src/
├── main.rs                  # Entry point
├── config.rs                # CatConfig struct (runtime configuration)
├── help.rs                  # Help/usage text
├── cat_core/
│   ├── mod.rs               # Module declarations
│   ├── runner.rs            # Execution orchestrator (file I/O dispatch)
│   ├── reader.rs            # Line-by-line processing and output
│   └── non_print.rs         # Non-printing character encoding (^X, M-x)
└── options/
    ├── mod.rs               # Module declarations
    ├── flags.rs             # Valid flag definitions
    └── evaluate.rs          # CLI argument parsing
```

## How It Works

1. **Argument parsing** (`options/evaluate.rs`) — Collects CLI args, validates flags, and populates a `CatConfig`. Invalid flags cause exit with code 2 (matching Unix convention).
2. **Dispatch** (`cat_core/runner.rs`) — If help is requested, prints help. Otherwise opens files (or stdin) and passes each to the line processor.
3. **Line processing** (`cat_core/reader.rs`) — Reads input line-by-line, applying transformations in order: non-printing character display, tab display, blank line squeezing, line numbering, and line-ending markers.

## License

MIT
