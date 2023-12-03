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
