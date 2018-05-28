//! Author @truexpixels
//!
//! Solution to ["Odd or Even" Problem](http://www.practicepython.org/exercise/2014/02/05/02-odd-or-even.html)
//!

use std::io;

fn main() {

    // Request User Input
    let number = input("Please input a number: ")
        .expect("Something went wrong!") // Catch Errors
        .parse::<i32>().expect("Invalid Number."); // If unable to parse to i32, output an error message

    // Output User Input
    println!("You wrote the number {}!", number);

    // Request user Input - Number to divide by
    let div_by = input("What number should I divide that by? ")
        .expect("Something went wrong!") // Catch Errors
        .parse::<i32>().expect("Invalid Number."); // If unable to parse to i32, output an error message

    // Add a break between text fields
    println!("");

    // Calculations
    if number % 2 == 0 { // If the remainder of the number divded by two is 0, that means it is even
        if number % 4 == 0 { // This will run if the number is a multiple of 4
            println!("This number is even, and a multiple of 4!")
        } else { // This will run if it isn't
            println!("This number is even!");
        }
    } else { // Although, if it's 1 it means that the number is odd
        println!("This number is odd!");
    }

    // Calculations - Divide by other number
    if number % div_by == 0 { // This will run if the first number divdes evenly into the second
        println!("{} divides evenly with {}!", number, div_by);
    } else { // This will run if there is a remainder
        println!("{} does not divide evenly with {}, there is a remainder of {}!", number, div_by, number % div_by)
    }

}

/// `input` mimics the input function in Python3
fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write; // Use Trait
    print!("{}", user_message); // Add message to print buffer
    io::stdout().flush()?; // Flush buffer (output everything in print)

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_right().to_owned())
}
