// I wrote this implementation of the solution for today as a learning exercise
// to get some experience with how Rust handles concurrency.
//
// In reality this is not a good use case for concurrency because the threads
// are all doing the same thing and they all need to access the same data.
// This means that the threads will spend most of their time waiting for the
// mutex to unlock.
// This would be less of an issue if the computation was more complex so that
// each thread took longer to complete, since the ratio of computation time to
// wait time would be higher.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time;
use std::sync::{Arc, Mutex};
use std::thread;

fn get_changes(data_points: &Vec<i64>) -> Vec<i64> {
    let mut changes: Vec<i64> = Vec::new();
    for i in 0..data_points.len() - 1 {
        changes.push(data_points[i + 1] - data_points[i]);
    }
    changes
}

fn get_next_value(data_points: &Vec<i64>) -> i64 {
    let changes = get_changes(data_points);
    let sum_of_changes = changes.iter().sum();
    match sum_of_changes {
        0 => data_points[0],
        _ => {
            let final_step = get_next_value(&changes);
            data_points[data_points.len() - 1] + final_step
        }
    }
}

fn solve_linear(lines: &Vec<String>) -> i64 {
    let mut next_values_sum: i64 = 0;
    for line in lines {
        let line_values = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let next_value = get_next_value(&line_values);
        next_values_sum += next_value;
    }
    next_values_sum
}

fn solve_concurrent_mutex(lines: &Vec<String>) -> i64 {
    // This is a concurrent implementation of the linear solution
    // It uses a mutex to lock the sum of the next values
    // ... and it takes about twice as long as the linear solution
    let next_values_sum: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));
    let mut threads = Vec::new();

    for line in lines {
        let line_values = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let next_values_sum = Arc::clone(&next_values_sum);
        let thread = thread::spawn(move || {
            let next_value = get_next_value(&line_values);
            let mut sum = next_values_sum.lock().unwrap();
            *sum += next_value;
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let x = *next_values_sum.lock().unwrap();
    x
}

fn main() {
    let file = File::open("../input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let start_time = time::SystemTime::now();
    let next_values_sum = solve_linear(&lines);
    println!("Sum of next values: {}", next_values_sum);
    println!("Time elapsed: {:?}", start_time.elapsed().unwrap());

    let start_time = time::SystemTime::now();
    let next_values_sum = solve_concurrent_mutex(&lines);
    println!("Sum of next values: {}", next_values_sum);
    println!("Time elapsed: {:?}", start_time.elapsed().unwrap());
}
