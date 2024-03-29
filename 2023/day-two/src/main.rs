use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    sequence::separated_pair,
    IResult,
};

fn main() {
    parse_line("Game 1: 3 blue, 4 red;").unwrap();
}

fn part_one(input: &str) -> usize {
    todo!()
}

#[derive(Debug, PartialEq)]
struct Hand {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

struct Game {
    pub hands: Vec<Hand>,
}

fn digit_parser(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn color_count_parser(input: &str) -> IResult<&str, (usize, &str)> {
    separated_pair(digit_parser, tag(" "), alpha1)(input)
}

impl Game {
    fn parse_line(input: &str) -> IResult<&str, Hand> {
        let (input, _) = tag("Game ")(input)?;
        let (input, round_number) = digit_parser(input)?;
        let (input, _) = tag(": ")(input)?;

        let mut hand_parser = map(color_count_parser, |(count, color)| Self {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
        });

        dbg!(round_number, input);

        todo!()
    }
}

mod test {
    #[test]
    fn initial_example_part_one() {
        use crate::part_one;
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(part_one(input), 8);
    }

    #[test]
    fn test_example_part_one() {
        use crate::part_one;
        let input = include_str!("./input.txt");
        assert_eq!(part_one(input), 8);
    }
}
