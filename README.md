# Cursor Mover

A simple Rust program that moves the mouse cursor randomly every minute for a user-specified duration. Useful for preventing idle states or keeping the screen active.

## Features

- Moves the mouse cursor to random screen coordinates.
- User-configurable duration (in minutes).
- Displays progress using a dynamic progress bar.
- Handles errors gracefully, such as invalid input or display issues.

## Requirements

This program requires the following Rust crates:

- [`enigo`](https://crates.io/crates/enigo) - For mouse control.
- [`indicatif`](https://crates.io/crates/indicatif) - For displaying a progress bar.
- [`rand`](https://crates.io/crates/rand) - For generating random coordinates.
- [`scrap`](https://crates.io/crates/scrap) - For retrieving screen dimensions.

## Installation

1. Make sure you have [Rust](https://www.rust-lang.org/) installed on your system.
2. Clone the repository or copy the program's source code into a Rust project.
3. Add the required dependencies to your `Cargo.toml` file:

```toml
[dependencies]
enigo = "0.0.14"
indicatif = "0.17"
rand = "0.8"
scrap = "0.5.0"
```

4. Build the project using `cargo build`.

## Usage

1. Run the program with the following command:

```bash
cargo run
```

2. Enter the desired number of minutes for the cursor movement.
3. The program will move the cursor randomly every 60 seconds and display progress.

### Example Output

```
Enter the number of minutes for moving the cursor:
5
Screen size: 1920x1080. Moving the cursor every 60 seconds for 5 minutes.
Press Ctrl+C to quit before finishing.
[00:01] #########>--------------------- 1/5 min.
...
Program completed.
```

## Notes

- The program calculates random coordinates within the screen's width and height.
- To stop the program before completion, press `Ctrl+C`.
