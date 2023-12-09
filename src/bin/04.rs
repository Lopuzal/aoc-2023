use std::collections::{HashMap, HashSet};
advent_of_code::solution!(4);


struct Card<'a> {
    winning_numbers: HashSet<&'a str>,
    numbers: Vec<&'a str>
}
pub fn part_one(input: &str) -> Option<u32> {
    let cards: Vec<Card> = input.lines()
        .map(|l| {l.split(':').collect::<Vec<_>>()[1].split('|')
            .map(|s| {s.trim()}).collect()})
        .map(|l:Vec<&str>| {Card{
            winning_numbers:l[0].split(' ').filter(|n|{!n.is_empty()}).collect(),
            numbers:l[1].split(' ').filter(|n|{!n.is_empty()}).collect()
        }}).collect();
    let mut totalpoints = 0u32;
    for card in cards {
        let mut card_score= 0u32;
        for number in card.numbers {
            print!("{:?}",card_score);
            if card.winning_numbers.contains(number){
                card_score = match card_score {
                    0 => 1,
                    x => x*2
                }
            }
        }
        totalpoints = totalpoints + card_score;
    }
    Some(totalpoints)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
