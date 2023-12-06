use itertools::Itertools;

pub(crate) fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (_label, data_str) = line
                .split_once(": ")
                .expect("scratchcard not splittable by label?");
            let (winning, own) = data_str
                .split_once(" | ")
                .expect("data on scratchcard not a splittable into two categories?");
            let (winning, own) = (
                winning
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
                own.split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            );
            let winning_cards = own.into_iter().filter(|num| winning.contains(num)).count() as u32;
            if winning_cards > 0 {
                2u32.saturating_pow(winning_cards - 1)
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    struct ScratchCardInfo {
        _label: usize,
        winning: Vec<u32>,
        owned: Vec<u32>,
    }
    // parse
    let cards = input
        .lines()
        .map(|line| {
            let (card_info, data) = line.split_once(": ").unwrap();
            let (_, label) = card_info.split_ascii_whitespace().collect_tuple().unwrap();
            let (winning, owned) = data.split_once(" | ").unwrap();
            let (winning, owned) = (
                winning
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
                owned
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            );
            ScratchCardInfo {
                _label: label.parse().unwrap(),
                winning,
                owned,
            }
        })
        .collect::<Vec<ScratchCardInfo>>();

    let mut scratchcards_total = 0u32; // originals counted by appearance in loop
    let mut card_amount_stack = vec![1u32; cards.len()];
    for card in cards {
        // dbg!(&card_amount_stack);
        let card_amount = card_amount_stack.pop().unwrap();
        scratchcards_total += card_amount;
        let winnings = card
            .owned
            .iter()
            .filter(|num| card.winning.contains(num))
            .count();
        if winnings > 0 {
            // necessary to keep away underflow of last_index on last card
            let last_index = card_amount_stack.len() - 1;
            // dbg!(card._label, scratchcards_total, winnings, last_index);
            for ahead_index in 0..winnings {
                let stack_index = last_index - ahead_index;
                card_amount_stack[stack_index] += card_amount;
                // eprintln!("added {} to index {}", card_amount, stack_index);
            }
        }
    }

    scratchcards_total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT), "13");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT), "30");
    }
}