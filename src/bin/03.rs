use std::io::Lines;
advent_of_code::solution!(3);
fn is_symbol(grid: &Vec<Vec<char>>, i: isize, j: isize) -> bool {
    match (usize::try_from(i),usize::try_from(j)){
        (Err(_),Err(_)) => false,
        (Ok(_),Err(_)) => false,
        (Err(_),Ok(_)) => false,
        (Ok(x),Ok(y)) => match grid.get(x){
            Some(line) => match line.get(y) {
                Some(char) => match char {
                    '0'..='9'=> false,
                    '.' => false,
                    _ => true,
                }
                None => false,
            }
            None => false,
        }
    }
}
fn is_number(symbol:char) -> bool {
    match symbol {
        '0'..='9'=> true,
        _ => false,
    }
}
fn symbol_neighbours(grid: &Vec<Vec<char>>, i: isize, j: isize) -> bool{
    is_symbol(&grid,i-1,j) || is_symbol(&grid,i-1,j+1) || is_symbol(&grid,i,j+1)
    || is_symbol(&grid,i,j-1) || is_symbol(&grid,i-1,j-1)
    || is_symbol(&grid,i+1,j) || is_symbol(&grid,i+1,j+1) || is_symbol(&grid,i+1,j-1)
}
pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| {l.chars().collect()}).collect();
    let mut part_number_sum = 0u32;
    let mut number: String= String::from("");
    let mut part_number_state = false;
    let mut number_state = false;

    for i in 0..lines.len(){
        for j in 0..lines[0].len(){
            if is_number(lines[i][j]){
                number_state = true;
                number = number + &*lines[i][j].to_string();
                part_number_state = part_number_state || symbol_neighbours(&lines,isize::try_from(i).unwrap(),isize::try_from(j).unwrap());
            } else if number_state{
                if part_number_state{part_number_sum = part_number_sum + number.parse::<u32>().unwrap();}

                number = String::from("");
                number_state = false;
                part_number_state = false;
            }
        }
        if part_number_state{part_number_sum = part_number_sum + number.parse::<u32>().unwrap();}
        number = String::from("");
        number_state = false;
        part_number_state = false;
    }
    Some(part_number_sum)
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
        assert_eq!(result, Some(4361u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
