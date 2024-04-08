use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

fn digit_parser(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

pub struct ScratchoffCard {
    card_num: usize,
    /// The first correct number scores 1 point, then each after that doubles the score.
    score: usize,
    winning_numbers: LotteryNums,
    my_numbers: LotteryNums,
}

impl ScratchoffCard {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tuple((tag("Card"), many1(tag(" "))))(input)?;
        let (input, card_num) = digit_parser(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, lottery_numbers) = separated_list1(
            tuple((many1(tag(" ")), tag("|"), many1(tag(" ")))),
            parse_lottery_numbers,
        )(input)?;

        assert_eq!(lottery_numbers.len(), 2);

        let winning_numbers = lottery_numbers[0].clone();
        let my_numbers = lottery_numbers[1].clone();

        let my_winning_numbers_count = my_numbers
            .iter()
            .filter(|num| winning_numbers.contains(num))
            .count();

        let score = Self::calc_score(my_winning_numbers_count);

        Ok((
            input,
            Self {
                card_num,
                winning_numbers,
                my_numbers,
                score,
            },
        ))
    }

    /// ```
    /// use day_4::part_one::ScratchoffCard;
    /// assert_eq!(ScratchoffCard::calc_score(0), 0);
    /// assert_eq!(ScratchoffCard::calc_score(1), 1);
    /// assert_eq!(ScratchoffCard::calc_score(2), 2);
    /// assert_eq!(ScratchoffCard::calc_score(3), 4);
    /// assert_eq!(ScratchoffCard::calc_score(4), 8);
    /// ```
    pub fn calc_score(winning_numbers: usize) -> usize {
        if winning_numbers == 0 {
            return 0;
        }
        if winning_numbers == 1 {
            return 1;
        }
        let mut score = 1;
        // Double the score for every additional winning number.
        for _ in 1..winning_numbers {
            score *= 2;
        }

        score
    }
}

type LotteryNums = Vec<usize>;

/// ```
/// use day_4::part_one::parse_lottery_numbers;
/// let input = "1 2 3 14 69 11";
/// assert_eq!(parse_lottery_numbers(input).unwrap(), ("", vec![1, 2, 3, 14, 69, 11]));
/// ```
pub fn parse_lottery_numbers(input: &str) -> IResult<&str, LotteryNums> {
    separated_list1(many1(tag(" ")), digit_parser)(input)
}

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| ScratchoffCard::parse(line).map(|card| card.1.score))
        .sum()
}

mod test {
    #[test]
    fn provided_testcase() {
        use crate::part_one::part_one;
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(part_one(input), 8);
    }

    #[test]
    fn provided_input() {
        use crate::part_one::part_one;
        let input = include_str!("./input.txt");
        assert_eq!(part_one(input), 8);
    }
}
