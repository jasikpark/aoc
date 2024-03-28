use std::usize;

fn main() {
    println!("Hello, world!");
}

pub fn part_one(input: &str) -> usize {
    input.split('\n').fold(0, |acc, line| {
        let first_index = line.find(char::is_numeric).unwrap();
        let last_index = line.rfind(char::is_numeric).unwrap();
        let first: String = line.chars().skip(first_index).take(1).collect();
        let last: String = line.chars().skip(last_index).take(1).collect();
        acc + (first + &last).parse::<usize>().unwrap()
    })
}

fn find_alpha_nums(input: &str) -> ((usize, &str), (usize, &str)) {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ];

    let x = nums.iter().flat_map(|num| input.match_indices(num));

    let first = x.clone().fold((usize::MAX, ""), |best_index, curr_index| {
        if curr_index.0 < best_index.0 {
            curr_index
        } else {
            best_index
        }
    });

    let last = x.fold((usize::MIN, ""), |best_index, curr_index| {
        if curr_index.0 > best_index.0 {
            curr_index
        } else {
            best_index
        }
    });

    (first, last)
}

fn numstring_tostring(input: &str) -> &str {
    match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "zero" => "0",
        _ => unreachable!("pls don't gib other options :3, ${:?}", input),
    }
}

pub fn part_two(input: &str) -> usize {
    input.split('\n').fold(0, |acc, line| {
        let first_index_numeric = line.find(char::is_numeric);
        let last_index_numeric = line.rfind(char::is_numeric);

        let (first_alphanum, last_alphanum) = find_alpha_nums(line);
        let first: String = if first_alphanum.1.is_empty()
            || first_index_numeric.is_some() && first_index_numeric.unwrap() < first_alphanum.0
        {
            let first_numeric: String = line
                .chars()
                .skip(first_index_numeric.unwrap())
                .take(1)
                .collect();
            first_numeric
        } else {
            String::from(numstring_tostring(first_alphanum.1))
        };
        let last: String = if last_alphanum.1.is_empty()
            || last_index_numeric.is_some() && last_index_numeric.unwrap() > last_alphanum.0
        {
            let last_numeric: String = line
                .chars()
                .skip(last_index_numeric.unwrap())
                .take(1)
                .collect();
            last_numeric
        } else {
            String::from(numstring_tostring(last_alphanum.1))
        };
        acc + (first + &last).parse::<usize>().unwrap()
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn provided_example_part_one() {
        use crate::part_one;
        let input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;
        assert_eq!(part_one(input), 142);
    }

    #[test]
    fn test_example_part_one() {
        use crate::part_one;
        let input = include_str!("./input.txt");
        assert_eq!(part_one(input), 54630);
    }

    #[test]
    fn provided_example_part_two() {
        use crate::part_two;
        let input = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;
        assert_eq!(part_two(input), 281);
    }

    #[test]
    fn test_example_part_two() {
        use crate::part_two;
        let input = include_str!("./input.txt");
        assert_eq!(part_two(input), 54770);
    }

    #[test]
    fn test_find_alpha_nums() {
        use crate::find_alpha_nums;
        let input = "onetwothree";
        insta::assert_debug_snapshot!(find_alpha_nums(input), @r###"
        (
            (
                0,
                "one",
            ),
            (
                6,
                "three",
            ),
        )
        "###);
    }
}
