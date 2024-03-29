use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    dbg!(Game::parse("Game 1: 3 blue, 4 red;").unwrap());
}

pub fn part_one(input: &str) -> usize {
    let games = input
        .lines()
        .map(|line| {
            let (input, game) = Game::parse(line.trim()).unwrap();
            assert!(input.is_empty());
            game
        })
        .collect::<Vec<_>>();

    dbg!(games);

    todo!();
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, PartialEq)]
struct ColorCount {
    pub count: usize,
    pub color: Color,
}

#[derive(Debug, PartialEq)]
struct Hand {
    pub color_counts: Vec<ColorCount>,
}

impl Hand {
    fn parse(input: &str) -> IResult<&str, Self> {
        let color_count_parser = map(Self::color_count_parser, |(count, color)| ColorCount {
            count,
            color: match color {
                "red" => Color::Red,
                "blue" => Color::Blue,
                "green" => Color::Green,
                _ => panic!("pls don't have other colors"),
            },
        });

        let (input, color_counts) = separated_list1(tag(", "), color_count_parser)(input)?;

        Ok((input, Self { color_counts }))
    }

    fn color_count_parser(input: &str) -> IResult<&str, (usize, &str)> {
        separated_pair(digit_parser, tag(" "), alpha1)(input)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    pub game_num: usize,
    pub hands: Vec<Hand>,
}

fn digit_parser(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Game ")(input)?;
        let (input, game_num) = digit_parser(input)?;
        let (input, _) = tag(": ")(input)?;

        let mut game_parser = separated_list1(tag("; "), Hand::parse);

        let (input, hands) = game_parser(input)?;

        Ok((input, Self { game_num, hands }))
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

    #[test]
    fn snapshot_game_parse() {
        use crate::Game;

        let input_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let input_2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let input_3 = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let input_4 = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let input_5 = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        insta::assert_debug_snapshot!(Game::parse(input_1));
        insta::assert_debug_snapshot!(Game::parse(input_2));
        insta::assert_debug_snapshot!(Game::parse(input_3));
        insta::assert_debug_snapshot!(Game::parse(input_4));
        insta::assert_debug_snapshot!(Game::parse(input_5));
    }
}
