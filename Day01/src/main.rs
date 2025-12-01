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

    let mut dial: i64 = 50;
    for (_line_num, line) in contents.lines().enumerate() {
        let rotation: i64 = line.split_at(1).1.parse::<i64>().unwrap() % 100;

        if line.starts_with('L') {
            dial -= rotation;
            if dial < 0 {
                dial += 100;
            }
        } else {
            dial += rotation;
            dial = dial % 100;
        }

        if dial == 0 {
            answer += 1;
        }
    }

    return answer;
}



#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;

    let mut dial: i64 = 50;
    let mut skip: bool = false;
    for (_line_num, line) in contents.lines().enumerate() {
        let mut rotation: i64 = line.split_at(1).1.parse::<i64>().unwrap();

        answer += rotation as u64 / 100;
        rotation %= 100;

        if line.starts_with('L') {
            dial -= rotation;
        } else {
            dial += rotation;
        }

        if dial == 0 || dial == 100 {
            answer += 1;
            skip = true;
        } else if (dial > 100 || dial < 0) && skip == false {
            answer += 1;
        } else {
            skip = false;
        }

        dial = (dial + 100) % 100;
    }

    return answer; // 6556 not right
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 3);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 6);
    }
}