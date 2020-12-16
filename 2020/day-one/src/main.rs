use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn find_it(nums: &Vec<i64>) -> Option<i64> {
    for num in nums {
        match nums.iter().find(|&&x| (*num + x) == 2020) {
            Some(x) => {
                println!("{}", num * x);
                return Some(num * x);
            }
            None => continue,
        }
    }
    return None;
}

fn find_it2(nums: &Vec<i64>) -> Option<i64> {
    for num in nums {
        for num2 in nums {
            match nums.iter().find(|&&x| (*num + *num2 + x) == 2020) {
                Some(x) => {
                    println!("{}", num * num2 * x);
                    return Some(num * num2 * x);
                }
                None => continue,
            }
        }
    }
    return None;
}

fn main() -> Result<(), Error> {
    let nums = read(File::open(
        "/Users/calebjasik/Desktop/Github/aoc/day-one/input",
    )?)?;
    println!(
        "Our number is: {}",
        find_it(&nums).expect("We didn't find the number")
    );
    println!(
        "Our number part 2 electric boogaloo is: {}",
        find_it2(&nums).expect("We didn't find the number")
    );
    Ok(())
}
