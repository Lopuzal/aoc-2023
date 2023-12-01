advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let numbers = lines.map(|l| {
        l.trim_matches(char::is_alphabetic)
    });
    let calibrations = numbers.map(|l|{
        [l.chars().nth(0).unwrap(), l.chars().last().unwrap()].iter().collect::<String>().parse::<u32>()
    });
    Some(calibrations.fold(0u32, |acc, x| acc + x.unwrap_or_default()))
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}