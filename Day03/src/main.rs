use std::fs;
use std::time::Instant;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut now: Instant = Instant::now();
    let part1: u64 = part1(contents.clone());
    let mut elapsed: std::time::Duration = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: u64 = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        let numerics: Vec<u64> = line.chars().map(|x: char| x.to_digit(10).unwrap() as u64).collect();
        let mut maximum: u64 = 0;
        for i in 1..line.len() {
            let d1: u64 = *numerics[..i].iter().max().unwrap();
            let d2: u64 = *numerics[i..].iter().max().unwrap();
            let test: u64 = d1*10 + d2;
            if test > maximum {
                maximum = test;
            }
        }
        answer += maximum;
    }

    return answer;
}



#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        let numerics: Vec<u64> = line.chars().map(|x: char| x.to_digit(10).unwrap() as u64).collect();
        // need to do a dfs on the numerics and run to completion
    }

    return answer;
}

#[allow(non_snake_case)]
fn dfs(numerics: Vec<u64>, digits: u64) -> u64 {
    let mut maximum: u64 = 0;
    let mut idx: Vec<u64> = vec![];
    let mut active_idx: u64 = digits-1;
    for i in 0..digits {
        idx.push(i);
    }

    while active_idx >= 0 {
        let mut number: u64 = 0;
        for i in idx.clone() {
            number = (number * 10) + numerics[i as usize];
            if number > maximum {
                maximum = number;
            }
        }

        if idx[active_idx as usize] + 1 < numerics.len() as u64 - (digits - active_idx) {
            idx[active_idx as usize] += 1;
        } else {
            // can't slide the active index any further right, set active index one further to the left
            active_idx
        }
    }

    return maximum;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 357);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 3121910778619);
    }
}