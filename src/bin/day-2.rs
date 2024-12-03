use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(Path::new("src/data/day-2.txt"))?;
    let reader = io::BufReader::new(file);
    let mut count1 = 0;
    let mut count2 = 0;

    for line in reader.lines() {
        let line = line?;
        let report = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        if validate(&report) {
            count1 += 1;
            count2 += 1;
        } else if (0..report.len()).any(|i| {
            let mut clone = report.clone();
            clone.remove(i);
            validate(&clone)
        }) {
            count2 += 1;
        }
    }

    println!("Part one: {}", count1);
    println!("Part two: {}", count2);

    Ok(())
}

fn validate(report: &[i32]) -> bool {
    let mut safe = true;
    let ascending = report[0] < report[1];
    for i in 0..report.len() - 1 {
        if !(1..=3).contains(&(report[i] - report[i + 1]).abs()) {
            safe = false;
        }
        if ascending && report[i] >= report[i + 1] {
            safe = false;
        }
        if !ascending && report[i] <= report[i + 1] {
            safe = false;
        }
    }

    safe
}
