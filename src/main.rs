use std::io;
use std::io::prelude::*;

fn print_count(key: &str, count: i64) {
    println!("{}\t{}", key, count);
}

fn main() {
    // Indicates whether we've ever seen a line. Ensures that we don't
    // output a spurious total at the start.
    let mut active: bool = false;

    // Some things we'll use repeatedly while iterating through lines.
    let stdin = io::stdin();

    // The current key that we'll be adding up counts for, as a string slice.
    // Meaningless when active == false.
    let mut key = String::new();

    // The count so far of the current key.
    let mut count: i64 = 0;

    for line_opt in stdin.lock().lines() {
        let line = line_opt.unwrap();

        // Split the line into a vector of two items: the value (which is at
        // the end of the line), and everything before it.
        let parts: Vec<&str> = line.rsplitn(2, '\t').collect();
        if parts.len() < 2 {
            // If we didn't get a tab-separated line, become inactive and
            // print it as-is.
            if active {
                print_count(&key, count);
                active = false;
            }
            println!("{}", line);
        }
        else {
            let this_key = parts[1].to_string();
            let this_opt: Option<i64> = parts[0].trim().parse().ok();
            match this_opt {
                Some(n) => {
                    // This line ends with a numeric value.
                    if active && this_key == key {
                        // This line's key matches the current key, so
                        // add the value to the total count.
                        count += n;
                    }
                    else {
                        // This line's key differs from the current key.
                        // Print the previous value if there is one.
                        if active {
                            print_count(&key, count);
                        }
                        // Start a new current key.
                        count = n;
                        key = this_key;
                        active = true;
                    }
                },
                None => {
                    // This line doesn't end with a numeric value, so become
                    // inactive and print it as-is.
                    if active {
                        print_count(&key, count);
                        active = false;
                    }
                    println!("{}", line);
                }
            }
        }
    }
    if active {
        print_count(&key, count);
    }
}
