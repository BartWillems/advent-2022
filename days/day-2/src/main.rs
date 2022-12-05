#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Copy, Clone)]
struct Round {
    elf: HandShape,
    you: HandShape,
}

struct Score(usize);

enum Direction {
    Lose,
    Draw,
    Win,
}

impl From<&str> for Direction {
    fn from(direction: &str) -> Self {
        match direction {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for HandShape {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => HandShape::Rock,
            "B" | "Y" => HandShape::Paper,
            "C" | "Z" => HandShape::Scissor,
            _ => unreachable!(),
        }
    }
}

// Part 2
impl From<(HandShape, Direction)> for Round {
    fn from(input: (HandShape, Direction)) -> Self {
        let (elf, you) = (input.0, input.1);

        let you = match (elf, you) {
            (elf, Direction::Draw) => elf,
            (HandShape::Rock, Direction::Lose) => HandShape::Scissor,
            (HandShape::Rock, Direction::Win) => HandShape::Paper,
            (HandShape::Paper, Direction::Lose) => HandShape::Rock,
            (HandShape::Paper, Direction::Win) => HandShape::Scissor,
            (HandShape::Scissor, Direction::Lose) => HandShape::Paper,
            (HandShape::Scissor, Direction::Win) => HandShape::Rock,
        };

        Round { elf, you }
    }
}

impl From<HandShape> for Score {
    fn from(shape: HandShape) -> Self {
        match shape {
            HandShape::Rock => Score(1),
            HandShape::Paper => Score(2),
            HandShape::Scissor => Score(3),
        }
    }
}

impl From<&Round> for Score {
    fn from(round: &Round) -> Self {
        match (round.elf, round.you) {
            (HandShape::Rock, HandShape::Paper) => Score(6),
            (HandShape::Rock, HandShape::Scissor) => Score(0),
            (HandShape::Paper, HandShape::Rock) => Score(0),
            (HandShape::Paper, HandShape::Scissor) => Score(6),
            (HandShape::Scissor, HandShape::Rock) => Score(6),
            (HandShape::Scissor, HandShape::Paper) => Score(0),
            _ => Score(3),
        }
    }
}

fn part_one(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| line.split_once(' '))
        .map(|round| match round {
            Some((elf, you)) => Round {
                elf: HandShape::from(elf),
                you: HandShape::from(you),
            },
            None => unreachable!(),
        })
        .map(|round| Score::from(&round).0 + Score::from(round.you).0)
        .sum()
}

fn part_two(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| line.split_once(' '))
        .map(|round| match round {
            Some((elf, you)) => (HandShape::from(elf), Direction::from(you)),
            None => unreachable!(),
        })
        .map(Round::from)
        .map(|round| Score::from(&round).0 + Score::from(round.you).0)
        .sum()
}

fn main() {
    let input = utils::load_file("inputs/day-2.txt");

    let score = part_one(&input);
    println!("part-1 score: {score}");

    let score = part_two(&input);
    println!("part-2 score: {score}");
}
