use enigo::{Coordinate::Abs, Enigo, Mouse, Settings};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use scrap::Display;
use std::{io, thread, time::Duration};

fn main() {
    // Prompt for the number of minutes
    println!("Enter the number of minutes for moving the cursor:");
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Error reading input. Please try again.");
        return;
    }

    let minutes: u32 = match input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("Please enter a valid positive number.");
            return;
        }
    };

    // Create an object to control the mouse
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Determine screen dimensions
    let display = match Display::primary() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: Failed to get the primary display. {e}");
            return;
        }
    };

    let (screen_width, screen_height) = (display.width() as i32, display.height() as i32);

    println!(
        "Screen size: {}x{}. Moving the cursor every 60 seconds for {} minutes.",
        screen_width, screen_height, minutes
    );
    println!("Press Ctrl+C to quit before finishing.");

    // Create a progress bar
    let pb = ProgressBar::new(minutes as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} min.")
            .unwrap()
            .progress_chars("#>-"),
    );

    pb.inc(0);

    // Main loop
    for _ in 0..minutes {
        thread::sleep(Duration::from_secs(60));

        // Generate random coordinates and move the cursor
        let (x, y) = (
            rand::thread_rng().gen_range(0..screen_width),
            rand::thread_rng().gen_range(0..screen_height),
        );
        enigo.move_mouse(x, y, Abs).unwrap();

        // Update the progress bar
        pb.inc(1);
    }

    pb.finish_with_message("Program completed.");
    println!("Program completed.");
}
