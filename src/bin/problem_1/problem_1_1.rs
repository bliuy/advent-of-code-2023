use std::{
    fs::File,
    io::{BufRead, BufReader},
};


/// Reads a file given its file path and returns a buffered reader.
///
/// # Arguments
///
/// * `fp` - A string slice that represents the file path.
///
/// # Returns
///
/// A `BufReader<File>` that allows efficient reading of the file.
fn read_file(fp: &str) -> BufReader<File> {
    // Printing the filepath to be read from
    println!("Reading file from the following filepath: {}", fp);

    // Opening the file
    let file = File::open(fp).expect("Unable to open file");

    // Reading the file into a BufReader
    let reader = BufReader::new(file);

    reader
}

/// Performs a naive search on the given sentence.
/// 
/// This function takes a reference to a string `sentence` and returns a tuple containing two characters.
/// The naive search algorithm searches for the first two distinct characters in the sentence and returns them in a tuple.
/// 
/// # Arguments
/// 
/// * `sentence` - The string to perform the search on.
/// 
/// # Returns
/// 
/// A tuple containing the first two distinct characters found in the sentence.
fn naive_search(sentence: &str) -> (char, char) {

    // Searching for the first digit
    let first_digit = sentence
        .chars()
        .find(|c| c.is_digit(10))
        .expect("Unable to find first digit");

    // Searching for the last digit
    let last_digit = sentence
        .chars()
        .rev()
        .find(|c| c.is_digit(10))
        .expect("Unable to find last digit");

    // Returning the first and last digits
    (first_digit, last_digit)
}

fn main() {
    // Reading the file
    let reader = read_file("src/bin/problem_1/inputs/input.txt");

    // Initializing the counter
    let mut counter = 0;

    // Starting the timer
    let start = std::time::Instant::now();

    // Printing the contents of the file
    for line in reader.lines() {
        // Reading the line into a string
        let sentence = line.expect("Unable to read line");

        // Performing the search
        let (first_digit, last_digit) = naive_search(&sentence);

        // Concatenating the results and converting into a number
        let result = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .expect("Unable to parse result");

        // Incrementing the counter
        counter += result;
    }

    // Stoping the timer
    let duration = start.elapsed();

    // Printing the result
    println!("The result is: {}", counter);
    println!("Time elapsed for the naive search algorithm: {:?}", duration);
}
