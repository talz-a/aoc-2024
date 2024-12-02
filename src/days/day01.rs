use std::{collections::HashMap, fs};

pub fn solve() {
    part_one();
    part_two();
}

fn parse_file() -> Result<(Vec<i32>, Vec<i32>), ()> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    fs::read_to_string("inputs/day1.txt")
        .expect("ERROR: could not find input file")
        .lines()
        .for_each(|line| {
            let mut parts = line.split_whitespace();
            let left_num = parts.next().expect("ERROR: could not parse input");
            let right_num = parts.next().expect("ERROR: could not parse input");
            left.push(left_num.parse::<i32>().unwrap());
            right.push(right_num.parse::<i32>().unwrap());
        });

    Ok((left, right))
}

fn part_two() {
    let (left, right) = parse_file().expect("ERROR: could not parse file");

    let right_freq = right
        .iter()
        .copied()
        .fold(HashMap::<i32, i32>::new(), |mut f, num| {
            f.entry(num).and_modify(|freq| *freq += 1).or_insert(1);
            f
        });

    let ans: i32 = left.iter()
        .map(|num| {
            let freq = *right_freq.get(num).unwrap_or(&0); 
            num * freq
        })
        .sum();

    println!("ANSWER: {ans}");
}

fn part_one() {
    let (mut left, mut right) = parse_file().expect("ERROR: could not parse file");
    left.sort();
    right.sort();

    let res =left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>();

    println!("ANSWER: {res}");
}
