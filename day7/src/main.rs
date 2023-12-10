use std::cmp;

static EXAMPLE_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 7 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    let start = std::time::Instant::now();
    part1(input);
    println!("Part 1: {}ms", start.elapsed().as_millis());
    // part2(input);
}

fn part1(input: &str) {
    let card_ranking = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let mut hands: Vec<(Vec<char>, usize)> = input
        .lines()
        .map(|line| {
            let mut cards = line.split_whitespace();
            let hand = cards.next().unwrap().chars().collect::<Vec<char>>();
            let rank = cards.next().unwrap().parse::<usize>().unwrap();
            (hand, rank)
        })
        .collect();

    hands.sort_by(|(hand1, _), (hand2, _)| {
        let hand1_rank = if try_with_every_joker(hand1, is_five_of_a_kind) {
            1
        } else if try_with_every_joker(hand1, is_four_of_a_kind) {
            2
        } else if try_with_every_joker(hand1, is_full_house) {
            3
        } else if try_with_every_joker(hand1, is_three_of_a_kind) {
            4
        } else if try_with_every_joker(hand1, is_two_pair) {
            5
        } else if try_with_every_joker(hand1, is_one_pair) {
            6
        } else if try_with_every_joker(hand1, is_high_card) {
            7
        } else {
            panic!("Unknown hand: {:?}", hand1);
        };

        let hand2_rank = if try_with_every_joker(hand2, is_five_of_a_kind) {
            1
        } else if try_with_every_joker(hand2, is_four_of_a_kind) {
            2
        } else if try_with_every_joker(hand2, is_full_house) {
            3
        } else if try_with_every_joker(hand2, is_three_of_a_kind) {
            4
        } else if try_with_every_joker(hand2, is_two_pair) {
            5
        } else if try_with_every_joker(hand2, is_one_pair) {
            6
        } else if try_with_every_joker(hand2, is_high_card) {
            7
        } else {
            panic!("Unknown hand: {:?}", hand2);
        };

        match hand1_rank.cmp(&hand2_rank) {
            cmp::Ordering::Less => cmp::Ordering::Less,
            cmp::Ordering::Greater => cmp::Ordering::Greater,
            cmp::Ordering::Equal => {
                if compare_equal_hands(hand1, hand2, &card_ranking) {
                    return cmp::Ordering::Greater;
                }
                cmp::Ordering::Less
            }
        }
    });

    let mut total = 0;
    hands
        .iter()
        .rev()
        .enumerate()
        .for_each(|(i, (hand, rank))| {
            let winnings = rank * (i + 1);
            total += winnings;
        });

    println!("{:?}", total);
}

// J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.
fn try_with_every_joker(hand: &Vec<char>, fn_to_try: fn(&Vec<char>) -> bool) -> bool {
    let hand = hand.clone();
    let possible_chars = hand
        .iter()
        .filter(|&&card| card != 'J')
        .collect::<Vec<&char>>();

    for (i, card) in hand.iter().enumerate() {
        if card == &'J' {
            for &possible_char in possible_chars.iter() {
                // replace all J with the possible char
                let hand_copy = hand.iter().collect::<String>();
                let hand_copy = hand_copy.replace('J', &possible_char.to_string());
                let hand_copy = hand_copy.chars().collect::<Vec<char>>();
                if fn_to_try(&hand_copy) {
                    return true;
                }
            }
        }
    }

    fn_to_try(&hand)
}

fn is_five_of_a_kind(hand: &Vec<char>) -> bool {
    let first_card = hand[0];
    hand.iter().all(|&card| card == first_card)
}

fn is_four_of_a_kind(hand: &Vec<char>) -> bool {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for &card in hand {
        *counts.entry(card).or_insert(0) += 1;
    }

    counts.values().any(|&count| count == 4)
}

fn is_full_house(hand: &Vec<char>) -> bool {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for &card in hand {
        *counts.entry(card).or_insert(0) += 1;
    }
    counts.values().any(|&count| count == 3) && counts.values().any(|&count| count == 2)
}

fn is_three_of_a_kind(hand: &Vec<char>) -> bool {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for &card in hand {
        *counts.entry(card).or_insert(0) += 1;
    }
    counts.values().any(|&count| count == 3)
}

fn is_two_pair(hand: &Vec<char>) -> bool {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for &card in hand {
        *counts.entry(card).or_insert(0) += 1;
    }
    counts.values().filter(|&&count| count == 2).count() == 2
}

fn is_one_pair(hand: &Vec<char>) -> bool {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for &card in hand {
        *counts.entry(card).or_insert(0) += 1;
    }
    counts.values().any(|&count| count == 2)
}

fn is_high_card(hand: &Vec<char>) -> bool {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for &card in hand {
        *counts.entry(card).or_insert(0) += 1;
    }
    counts.values().all(|&count| count == 1)
}

fn compare_equal_hands(hand1: &[char], hand2: &[char], card_rankings: &[char]) -> bool {
    for i in 0..5 {
        let card1 = hand1[i];
        let card2 = hand2[i];
        let card1_rank = card_rankings
            .iter()
            .position(|&card| card == card1)
            .unwrap();
        let card2_rank = card_rankings
            .iter()
            .position(|&card| card == card2)
            .unwrap();
        match card1_rank.cmp(&card2_rank) {
            cmp::Ordering::Less => return false,
            cmp::Ordering::Greater => return true,
            cmp::Ordering::Equal => continue,
        }
    }

    false
}

fn part2(input: &str) {
    todo!()
}
