use std::{io, thread};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, mpsc};

use regex::Regex;
use tqdm::tqdm;

fn main() {
    let test_file = "./data/test_cases.txt";
    let dictionary_file = "./data/dictionary.txt";

    if let Err(e) = evaluate_tests(test_file, dictionary_file) {
        eprintln!("Error: {}", e);
    }
}

// 评估所有测试用例并计算正确率
fn evaluate_tests(test_file: &str, dictionary_file: &str) -> io::Result<()> {
    let words = load_dictionary(dictionary_file)?;

    let file = File::open(test_file)?;
    let reader = BufReader::new(file);
    let mut test_cases = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            let parts: Vec<&str> = line.trim().split(", ").collect();
            if parts.len() == 2 {
                let input_letters = parts[0].to_string();
                let expected_result = parts[1].split(':').nth(1).unwrap_or("").to_string();
                test_cases.push((input_letters, expected_result));
            }
        }
    }
    println!("Finished pushing test cases into vector...");

    let total_count = test_cases.len();
    let (tx, rx) = mpsc::channel();
    let words = Arc::new(words);

    println!("Start processing test cases...");
    for test_case in tqdm(test_cases) {
        let words = Arc::clone(&words);
        let tx = tx.clone();
        thread::spawn(move || {
            let result = run_test_case(&test_case, &words);

            // 捕获可能的错误，避免死锁或未处理的panic
            if tx.send(result).is_err() {
                eprintln!("Error sending result via channel");
            }
        });
    }

    drop(tx); // 确保所有发送者都已完成发送，避免 `recv` 永远等待

    let mut correct_count = 0;
    println!("Calculating result...");
    for result in tqdm(rx.iter().take(total_count)) {
        if result {
            correct_count += 1;
        }
    }

    let accuracy = (correct_count as f64 / total_count as f64) * 100.0;
    println!("正确率: {:.2}%", accuracy);

    Ok(())
}

// 运行单个测试用例并返回是否正确
fn run_test_case(test_case: &(String, String), words: &Vec<String>) -> bool {
    let (input_letters, expected_result) = test_case;
    let letter_count_re = Regex::new(r"(\w+):(\d+)").unwrap();
    let mut letter_count = HashMap::new();

    for cap in letter_count_re.captures_iter(input_letters) {
        let letter = cap[1].to_string();
        let count: u32 = cap[2].parse().unwrap();
        letter_count.insert(letter, count);
    }

    if let Some(best_match) = find_best_match(words, &letter_count) {
        best_match == *expected_result
    } else {
        false
    }
}

fn get_word_score(word: String, letter_count: &HashMap<String, u32>) -> u32 {
    let mut score = 0;
    let mut word_count = HashMap::new();
    for letter in word.chars() {
        let entry = word_count.entry(letter.to_string()).or_insert(0);
        *entry += 1
    }
    for (letter, count) in word_count.iter() {
        if let Some(&letter_count) = letter_count.get(letter) {
            score += count.min(&letter_count)
        }
    }
    score
}

fn find_best_match(words: &Vec<String>, letter_count: &HashMap<String, u32>) -> Option<String> {
    let total_letter_count: u32 = letter_count.values().sum();
    let mut best_match = None;
    let mut best_score = 0;
    let mut closest_difference = u32::MAX;
    for word in words {
        let score = get_word_score(word.clone(), letter_count);
        let word_letter_count = word.len() as u32;
        let difference = (word_letter_count as i32 - total_letter_count as i32).abs() as u32;

        if score > best_score || (score == best_score && difference < closest_difference) {
            best_score = score;
            best_match = Some(word.clone());
            closest_difference = difference;
        }
    }
    best_match
}

fn load_dictionary(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let render = BufReader::new(file);

    let mut words = Vec::new();
    for line in render.lines() {
        let line = line?;
        words.push(line.trim().to_string());
    }
    Ok(words)
}

#[cfg(test)]
mod tests {
    use crate::load_dictionary;

    #[test]
    fn test_load_dictionary() {
        let path: &str = "./data/dictionary.txt";
        let words = load_dictionary(path).expect("Error reading words");
        assert_eq!(words.first().unwrap(), "A");
    }
}