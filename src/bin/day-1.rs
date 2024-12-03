use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(Path::new("src/data/day-1.txt"))?;
    let reader = io::BufReader::new(file);
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in reader.lines() {
        let line = line?;
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort();
    right.sort();

    let mut distance: i32 = 0;
    let mut similarity: i32 = 0;
    for n in 0..left.len() {
        distance += (left[n] - right[n]).abs();
        let count = right.iter().filter(|&x| *x == left[n]).count() as i32;
        similarity += left[n] * count
    }

    println!("Part one: {}", distance);
    println!("Part two: {}", similarity);

    Ok(())
}
