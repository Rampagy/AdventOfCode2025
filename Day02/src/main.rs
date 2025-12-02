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
        let ranges: Vec<&str> = line.split(',').collect();

        for range in ranges {
            let result: Vec<u64> = range.split('-').map(|x: &str| x.parse::<u64>().unwrap()).collect();
            for i in *result.first().unwrap()..=*result.last().unwrap() {
                let i_str: String = i.to_string();
                let sides: (&str, &str) = i_str.split_at(i_str.len()/2);
                if sides.0 == sides.1 && !sides.1.starts_with('0') {
                    answer += i;
                }
            }
        }
    }

    return answer;
}



#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut answer: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        let ranges: Vec<&str> = line.split(',').collect();

        for range in ranges {
            let result: Vec<u64> = range.split('-').map(|x: &str| x.parse::<u64>().unwrap()).collect();
            for i in *result.first().unwrap()..=*result.last().unwrap() {
                let i_str: String = i.to_string();

                for j in 1..=(i_str.len()/2) {
                    let (pattern, remainder) = i_str.split_at(j);
                    if remainder.trim_start_matches(pattern).len() == 0 {
                        answer += i;
                        break;
                    }
                }
            }
        }
    }

    return answer;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 1227775554);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 4174379265);
    }
}