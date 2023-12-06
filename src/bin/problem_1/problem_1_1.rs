use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file(fp: &str) -> BufReader<File> {
    // Printing the filepath to be read from
    println!("Reading file from the following filepath: {}", fp);

    // Opening the file
    let file = File::open(fp).expect("Unable to open file");

    // Reading the file into a BufReader
    let reader = BufReader::new(file);

    reader
}

// Implementing the search function
// This is a naive search function
// It is not very efficient, but it is easy to understand
fn naive_search(sentence: &str) -> (char, char) {
    // Printing the sentence to be searched
    println!("Searching the following sentence: {}", sentence);

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

    let mut counter = 0;

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

    // Printing the result
    println!("The result is: {}", counter);
}
