use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::{many0, many1, separated_list1},
    sequence::tuple,
    IResult,
};

fn digit_parser(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

#[derive(Debug)]
pub struct ScratchoffCard {
    card_num: usize,
    winning_numbers: LotteryNums,
    my_numbers: LotteryNums,
}

impl ScratchoffCard {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tuple((many0(tag(" ")), tag("Card"), many1(tag(" "))))(input)?;
        let (input, card_num) = digit_parser(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, lottery_numbers) = separated_list1(
            tuple((many0(tag(" ")), tag("|"), many0(tag(" ")))),
            parse_lottery_numbers,
        )(input)?;

        assert_eq!(lottery_numbers.len(), 2);

        let winning_numbers = lottery_numbers[0].clone();
        let my_numbers = lottery_numbers[1].clone();

        let my_winning_numbers_count = my_numbers
            .iter()
            .filter(|num| winning_numbers.iter().any(|winner| winner == *num))
            .count();

        Ok((
            input,
            Self {
                card_num,
                winning_numbers,
                my_numbers,
            },
        ))
    }
}

type LotteryNums = Vec<usize>;

/// ```
/// use day_4::part_one::parse_lottery_numbers;
/// let input = " 1 2 3 14 69 11";
/// assert_eq!(parse_lottery_numbers(input).unwrap(), ("", vec![1, 2, 3, 14, 69, 11]));
/// ```
pub fn parse_lottery_numbers(input: &str) -> IResult<&str, LotteryNums> {
    let (input, (_, nums, _)) = tuple((
        many0(tag(" ")),
        separated_list1(many1(tag(" ")), digit_parser),
        many0(tag(" ")),
    ))(input)?;

    Ok((input, nums))
}

pub fn part_two(input: &str) -> usize {
    let initial_cards = input
        .lines()
        .map(|line| ScratchoffCard::parse(line).unwrap().1)
        .collect::<Vec<_>>();

    dbg!(initial_cards);

    todo!();
}

mod test {
    #[test]
    fn provided_testcase() {
        use crate::part_two::part_two;
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        // 30 scratchcards
        assert_eq!(part_two(input), 30);
    }

    #[test]
    fn provided_input() {
        use crate::part_two::part_two;
        let input = include_str!("./input.txt");
        assert_eq!(part_two(input), 0);
    }
}
