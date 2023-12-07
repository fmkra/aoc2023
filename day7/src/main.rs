use std::{fs, cmp::Ordering};


#[derive(Eq)]
struct Hand {
    cards: [u8; 5],
    rank: u32,
}

#[derive(Debug)]
#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,       
}

impl Ord for Hand {
    fn cmp(&self, other:&Self) -> Ordering {
        let type1 = self.get_type();
        let type2 = other.get_type();
        if type1 != type2 {
            return type1.cmp(&type2);
        }
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].cmp(&other.cards[i]);
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let type1 = self.get_type();
        let type2 = other.get_type();
        if type1 != type2 {
            return false;
        }
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        true
    }
}

fn get_type_no_joker(cards: [u8; 5]) -> HandType {
    let mut card_count = [0; 13];
    for card in cards {
        card_count[card as usize] += 1;
    }
    let mut n_pair_count: [i32; 5] = [0; 5];
    for count in card_count {
        if count > 0 {
            n_pair_count[count-1 as usize] += 1;
        }
    }
    if n_pair_count[4] == 1 {
        return HandType::FiveOfKind;
    }
    if n_pair_count[3] == 1 {
        return HandType::FourOfKind;
    }
    if n_pair_count[2] == 1 && n_pair_count[1] == 1 {
        return HandType::FullHouse;
    }
    if n_pair_count[2] == 1 {
        return HandType::ThreeOfKind;
    }
    if n_pair_count[1] == 2 {
        return HandType::TwoPair;
    }
    if n_pair_count[1] == 1 {
        return HandType::OnePair;
    }
    return HandType::HighCard;
}

impl Hand {
    fn char_to_card(c: char) -> u8 {
        match c {
            'J' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("Invalid card"),
        }
    }

    fn new(hand: String, rank: u32) -> Hand {
        Hand {
            cards: [
                Hand::char_to_card(hand.chars().nth(0).unwrap()),
                Hand::char_to_card(hand.chars().nth(1).unwrap()),
                Hand::char_to_card(hand.chars().nth(2).unwrap()),
                Hand::char_to_card(hand.chars().nth(3).unwrap()),
                Hand::char_to_card(hand.chars().nth(4).unwrap()),
            ],
            rank: rank,
        }
    }

    fn get_type(&self) -> u8 {
        let mut cards = self.cards.clone();
        let mut max_hand_type = 0;
        for j1 in 1..=12 {
            if cards[0] != 0 && j1 != 1 {
                continue;
            }
            let mut j1s = false;
            if cards[0] == 0 {
                cards[0] = j1;
                j1s = true;
            }
            for j2 in 1..=12 {
                if cards[1] != 0 && j2 != 1 {
                    continue;
                }
                let mut j2s = false;
                if cards[1] == 0 {
                    cards[1] = j2;
                    j2s = true;
                }
                for j3 in 1..=12 {
                    if cards[2] != 0 && j3 != 1 {
                        continue;
                    }
                    let mut j3s = false;
                    if cards[2] == 0 {
                        cards[2] = j3;
                        j3s = true;
                    }
                    for j4 in 1..=12 {
                        if cards[3] != 0 && j4 != 1 {
                            continue;
                        }
                        let mut j4s = false;
                        if cards[3] == 0 {
                            cards[3] = j4;
                            j4s = true;
                        }
                        for j5 in 1..=12 {
                            if cards[4] != 0 && j5 != 1 {
                                continue;
                            }
                            let mut j5s = false;
                            if cards[4] == 0 {
                                cards[4] = j5;
                                j5s = true;
                            }
                            let hand_type = get_type_no_joker(cards) as u32;
                            if hand_type > max_hand_type {
                                max_hand_type = hand_type;
                            }
                            if j5s {
                                cards[4] = 0;
                            }
                        }
                        if j4s {
                            cards[3] = 0;
                        }
                    }
                    if j3s {
                        cards[2] = 0;
                    }
                }
                if j2s {
                    cards[1] = 0;
                }
            }
            if j1s {
                cards[0] = 0;
            }
        }
        max_hand_type as u8
    }
}

fn main() {
    let file = fs::read_to_string("data.txt").expect("Unable to read file");

    let mut cards = file.lines().map(|line| {
        let splited = line.split(" ").collect::<Vec<&str>>();
        let (hand, rank) = (splited[0], splited[1].parse::<u32>().unwrap());
        Hand::new(hand.to_string(), rank)
    }).collect::<Vec<_>>();

    // cards.iter().for_each(|hand| println!("{:?} {}", hand.cards, hand.get_type()));
    cards.sort();

    let sum: u32 = cards.iter().enumerate().map(|(i, hand)| (i+1) as u32 * hand.rank).sum();
    println!("sum: {}", sum);
}
