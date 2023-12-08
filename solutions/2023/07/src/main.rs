use std::fs;

#[derive(Debug)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Ord, Eq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Should not get here"),
        }
    }
}

fn card_to_points(card: Card) -> usize {
    match card {
        Card::Two => 2,
        Card::Three => 3,
        Card::Four => 4,
        Card::Five => 5,
        Card::Six => 6,
        Card::Seven => 7,
        Card::Eight => 8,
        Card::Nine => 9,
        Card::Ten => 10,
        Card::Jack => 11,
        Card::Queen => 12,
        Card::King => 13,
        Card::Ace => 14,
    }
}
#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    score: usize,
}

impl Hand {
    fn new(cards: &[Card], bid: usize) -> Self {
        let score = Self::score(cards);
        Self {
            cards: cards.to_vec(),
            bid,
            score,
        }
    }

    fn score(cards: &[Card]) -> usize {
        let mut sorted: Vec<Card> = cards.into();

        sorted.sort();

        let hand_type = if sorted[0] == sorted[4] {
            HandType::FiveOfKind
        } else if sorted[0] == sorted[3] || sorted[1] == sorted[4] {
            HandType::FourOfKind
        } else if sorted[0] == sorted[1] && sorted[2] == sorted[4]
            || sorted[0] == sorted[2] && sorted[3] == sorted[4]
        {
            HandType::FullHouse
        } else if sorted[0] == sorted[2] || sorted[1] == sorted[3] || sorted[2] == sorted[4] {
            HandType::ThreeOfKind
        } else {
            let pairs = sorted
                .as_slice()
                .windows(2)
                .filter(|pair| pair[0] == pair[1])
                .count();
            match pairs {
                0 => HandType::HighCard,
                1 => HandType::OnePair,
                2 => HandType::TwoPair,
                _ => panic!("Should not reach here"),
            }
        };

        let mut score = hand_type as usize + 1;
        for card in cards {
            score = score * 100 + card_to_points(card.clone()) as usize;
        }
        score
    }
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let input_split: Vec<&str> = input.split("\n").collect();

    part1(&input_split);

    Ok(())
}

fn part1(input: &Vec<&str>) {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input {
        let (hand, bid) = line.split_once(" ").unwrap();
        let bid: usize = bid.parse().unwrap();

        let cards: Vec<Card> = hand.chars().map(|c| Card::from_char(c)).collect();
        let hand = Hand::new(&cards[..], bid);
        hands.push(hand)
    }

    hands.sort_by(|h1, h2| h1.score.cmp(&h2.score));
    for (idx, hand) in hands.iter().enumerate() {
        println!("{:}: {hand:?} -> {}", (idx + 1), (idx + 1) * hand.bid);
    }
    let final_score = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum::<usize>();
    println!("Final score: {}", final_score);
}
