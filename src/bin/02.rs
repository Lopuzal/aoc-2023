use std::cmp::{max, min};
advent_of_code::solution!(2);

struct Game<'a> {
    id: u32,
    data: Vec<&'a str>,
}

fn format_game(game_input: &str) -> Game {
    let game: Vec<_> = game_input.split(':').collect();
    let game_id = game[0].replace("Game ","").parse().unwrap();
    let game_data = game[1].split(";").collect();
    Game { id : game_id, data : game_data}
}

fn draw_is_possible(draw: &Vec<&str>, maxblue: u32, maxred: u32, maxgreen: u32) -> bool {
    match draw[..] {
        [x,"blue"] => x.parse::<u32>().unwrap() <= maxblue,
        [x,"red"] => x.parse::<u32>().unwrap() <= maxred,
        [x,"green"] => x.parse::<u32>().unwrap() <= maxgreen,
        _ => false,
    }

}

fn set_is_possible(set: &str, maxblue: u32, maxred: u32, maxgreen: u32) -> bool {
    let cubes: Vec<&str> = set.split(',').collect();
    let cubes: Vec<Vec<&str>> = cubes.iter().map(|c| {c.trim().split(' ').collect()}).collect();
    cubes.iter().map(|c| {draw_is_possible(c,maxblue,maxred,maxgreen)}).fold(true, |acc,x| acc && x)
}

fn game_is_possible(game: &&Game,maxblue: u32, maxred: u32, maxgreen: u32) -> bool {
    let game_data = &game.data;
    game_data.iter().map(|s| {set_is_possible(s, maxblue, maxred, maxgreen)}).fold(true, |acc, x| acc && x)
}
pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let games: Vec<Game> = lines.map(|l| {format_game(l)}).collect();
    let valid_games: Vec<&Game> = games.iter().filter(|g| {game_is_possible(g,14u32,12u32,13u32)}).collect();
    Some(valid_games.iter().fold(0u32,|acc,x| acc + x.id))
}

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn maximum(&self, _set: Set) -> Set{
        Set {
            red : max_int_or_none(Some(self.red), Some(_set.red)),
            green : max_int_or_none(Some(self.green), Some(_set.green)),
            blue : max_int_or_none(Some(self.blue), Some(_set.blue)),
        }
    }
    fn power(&self) -> u32{
        self.red * self.green * self.blue
    }
}

fn create_set_from_drawstr(draw: &Vec<&str>) -> Set {
    match draw[..] {
        [x,"red"] => Set {red: x.parse::<u32>().unwrap(), green: 0u32, blue: 0u32},
        [x,"green"] => Set {red: 0u32, green: x.parse::<u32>().unwrap(), blue: 0u32},
        [x,"blue"] => Set {red: 0u32, green: 0u32, blue: x.parse::<u32>().unwrap()},
        _ => Set {red: 0u32, green: 0u32, blue: 0u32},
    }

}

fn create_set_from_setstr(set: &str) -> Set {
    let cubes: Vec<&str> = set.split(',').collect();
    let cubes: Vec<Vec<&str>> = cubes.iter().map(|c| {c.trim().split(' ').collect()}).collect();
    cubes.iter().map(|c| {create_set_from_drawstr(c)}).fold(Set {red: 0u32, green: 0u32, blue: 0u32}, |acc,x| acc.maximum(x))
}
fn power_minimal_game(game: &Game) -> u32 {
    let game_data = &game.data;
    let _set = game_data.iter().map(|s| {create_set_from_setstr(s)}).fold(Set {red: 0u32, green: 0u32, blue: 0u32}, |acc,x| acc.maximum(x));
    _set.power()
}

fn max_int_or_none(a: Option<u32>, b: Option<u32>) -> u32{
    match (a,b) {
        (Some(x),None) => x,
        (None,Some(x)) => x,
        (None,None) => 0,
        (Some(x),Some(y)) => max(x,y),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let games: Vec<Game> = lines.map(|l| {format_game(l)}).collect();
    let powers: Vec<u32> = games.iter().map(|g| {power_minimal_game(g)}).collect();
    Some(powers.iter().fold(0u32,|acc,x| acc + x))

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
