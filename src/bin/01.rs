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

fn unspell(line: &str) -> String {
    line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let unspelled_lines = lines.map(|l| unspell(l));
    let numbers = unspelled_lines.map(|l| {
        l.trim_matches(char::is_alphabetic).to_owned()
    });
    let calibrations = numbers.map(|l|{
        [l.chars().nth(0).unwrap(), l.chars().last().unwrap()].iter().collect::<String>().parse::<u32>()
    });
    Some(calibrations.fold(0u32, |acc, x| acc + x.unwrap_or_default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result,Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result,Some(281));
    }
}