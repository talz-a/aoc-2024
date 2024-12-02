use std::fs;

pub fn solve() {
    part_one();
    part_two();
}

fn parse_input(file_path: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(file_path)
        .expect("ERROR: could not find input file")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Invalid number in input"))
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_valid(arr: &[i32]) -> bool {
    let increasing = arr[0] <= arr[1]; 
    arr.windows(2).all(|pair| {
        let x = pair[0];
        let y = pair[1];
        let diff = (x - y).abs();
        diff >= 1 && diff <= 3 && if increasing { x < y } else { x > y }
    })
}

fn part_one() {
    let input = parse_input("inputs/day2.txt");
    let res: u32 = input
        .into_iter()
        .map(|nums| is_valid(&nums)) 
        .map(|v| v as u32)
        .sum();
    println!("ANSWER: {res}");
}

fn part_two() {
    let input = parse_input("inputs/day2.txt");
    let res: u32 = input
        .into_iter()
        .map(|nums| {
            if is_valid(&nums) {
                return true;
            }
            nums.iter().enumerate().any(|(i, _)| {
                is_valid(
                    &nums
                        .iter()
                        .enumerate()
                        .filter_map(|(j, &x)| if j != i {Some(x)} else {None})
                        .collect::<Vec<i32>>()
                )
            })
        })
        .map(|v| v as u32)
        .sum();
    println!("ANSWER: {res}");
}
