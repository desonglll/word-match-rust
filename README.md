# Word Matching Project (Rust)

## Overview

This project is a Rust implementation that takes a list of words and evaluates the best match for a given set of letter
counts. The system compares words from a dictionary to determine which one matches best based on the input letter
distribution, and computes the accuracy of the matching process through multiple test cases.

### Key Features

- **Multi-threading**: The project processes multiple test cases concurrently using Rust's threading system for
  efficiency.
- **Progress Tracking**: The `tqdm` crate is used to track the progress of test case processing and result calculation.
- **Regex Matching**: The project uses regular expressions to parse and process input data.
- **Concurrency Management**: The project leverages `Arc` for safe shared access to the dictionary across multiple
  threads and channels for managing thread communication.

## File Structure

```plaintext
├── data/
│   ├── dictionary.txt       # A file containing a list of words (one word per line)
│   └── test_cases.txt       # A file containing test cases with input letters and expected results
├── src/
│   ├── main.rs              # Main code that contains the word matching logic and multithreaded execution
├── README.md                # This README file
├── Cargo.toml               # Configuration file for managing dependencies
```

## Dependencies

This project uses the following external crates:

- [Regex](https://crates.io/crates/regex): For pattern matching and extracting letter counts from input strings.
- [Tqdm](https://crates.io/crates/tqdm): For displaying progress bars during the processing of test cases.

To add these dependencies to your project, include them in your `Cargo.toml` file like this:

```toml
[dependencies]
regex = "1.5"
tqdm = "0.3"
```

## How to Use

### Step 1: Prepare Your Data Files

1. **dictionary.txt**: This file contains a list of words. Each word should be on its own line. For example:

```
apple
banana
orange
```

2. **test_cases.txt**: This file contains test cases in the following format:

```
a:2, p:2, l:1, e:1, expected:apple
b:1, a:2, n:2, expected:banana
```

Each line consists of the input letter counts, followed by the expected result word.

### Step 2: Compile and Run

To compile the project, run:

```bash
cargo build
```

Then, to execute the program:

```bash
cargo run
```

### Step 3: Evaluate Test Cases

The program will automatically load the dictionary and test cases, and begin evaluating them using multiple threads. It
will display progress bars for both test case processing and result calculation.

### Example Output

```
Finished pushing test cases into vector...
Start processing test cases...
Processing: 100%|████████████████████| 100/100 [00:05<00:00, 19.47it/s]
Calculating result...
Accuracy: 95.00%
```

### Step 4: Run Unit Tests

The project also includes basic unit tests. To run the tests:

```bash
cargo test
```

## Functions

### `load_dictionary`

This function loads a list of words from a file and returns them as a `Vec<String>`.

### `evaluate_tests`

Reads test cases from a file and evaluates them using a multithreaded approach. It processes each test case concurrently
and calculates the overall accuracy.

### `run_test_case`

Takes a single test case and a list of words, computes the best match, and returns whether the result matches the
expected outcome.

### `find_best_match`

Finds the word from the dictionary that best matches the given letter counts.

### `get_word_score`

Calculates a score for how well a word matches the input letter counts.
