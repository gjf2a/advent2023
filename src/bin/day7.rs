use std::{cmp::Ordering, str::FromStr};

use advent_code_lib::{all_lines, chooser_main, Part};
use enum_iterator::{all, Sequence};
use hash_histogram::HashHistogram;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut hands_with_bids = hands_with_bids(filename)?;
        if part == Part::Two {
            for (hand, _) in hands_with_bids.iter_mut() {
                hand.use_joker = true;
            }
        }
        hands_with_bids.sort();
        let score = hands_with_bids
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| *bid * (i + 1) as u64)
            .sum::<u64>();
        println!("Score: {score}");
        Ok(())
    })
}

fn hands_with_bids(filename: &str) -> anyhow::Result<Vec<(Hand, u64)>> {
    let mut result = vec![];
    for line in all_lines(filename)? {
        let mut line_parts = line.split_whitespace();
        let hand = line_parts.next().unwrap().parse::<Hand>()?;
        let bid = line_parts.next().unwrap().parse::<u64>()?;
        result.push((hand, bid));
    }
    Ok(result)
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandLevel {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord)]
struct Hand {
    cards: [Card; 5],
    use_joker: bool,
}

impl Hand {
    fn all_joker_variants(&self) -> Vec<Hand> {
        all::<Card>().map(|c| self.replace_jokers_with(c)).collect()
    }

    fn replace_jokers_with(&self, sub: Card) -> Hand {
        let mut replaced = self.clone();
        replaced.use_joker = false;
        for card in replaced.cards.iter_mut() {
            if *card == Card::Jack {
                *card = sub;
            }
        }
        replaced
    }

    fn level(&self) -> HandLevel {
        if self.use_joker {
            return self
                .all_joker_variants()
                .iter()
                .map(|v| v.level())
                .max()
                .unwrap();
        }
        let mut hist = HashHistogram::new();
        for card in self.cards.iter() {
            hist.bump(card);
        }
        let ranking = hist.ranking_with_counts();
        match ranking[0].1 {
            5 => HandLevel::FiveOfAKind,
            4 => HandLevel::FourOfAKind,
            3 => match ranking[1].1 {
                2 => HandLevel::FullHouse,
                _ => HandLevel::ThreeOfAKind,
            },
            2 => match ranking[1].1 {
                2 => HandLevel::TwoPair,
                _ => HandLevel::OnePair,
            },
            _ => HandLevel::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let level_cmp = self.level().cmp(&other.level());
        match level_cmp {
            std::cmp::Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    if self.use_joker {
                        if *self_card == Card::Jack && *other_card != Card::Jack {
                            return Some(Ordering::Less);
                        } else if *self_card != Card::Jack && *other_card == Card::Jack {
                            return Some(Ordering::Greater);
                        }
                    }
                    let card_cmp = self_card.cmp(other_card);
                    if card_cmp != Ordering::Equal {
                        return Some(card_cmp);
                    }
                }
                Some(Ordering::Equal)
            }
            _ => Some(level_cmp),
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 5 {
            let mut cards = [Card::Two; 5];
            for (i, c) in s.chars().enumerate() {
                cards[i] = Card::try_from(c)?;
            }
            Ok(Self {
                cards,
                use_joker: false,
            })
        } else {
            Err(anyhow::anyhow!("Hand contains {} cards, not 5", s.len()))
        }
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Hash, Sequence)]
enum Card {
    #[default]
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

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> anyhow::Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(anyhow::anyhow!("Unmatched character: '{value}'")),
        }
    }
}
