use day_02::game::{game::Game, round::Round};

fn get_id_from_game_info(game_info: &str) -> Result<i32, &'static str> {
    let parts = game_info.split(' ').collect::<Vec<&str>>();
    let id = parts
        .get(1)
        .expect("Couldn't retrieve string id from game_info");
    let parsed_id = id.parse::<i32>().expect("Couln't parse game id");
    Ok(parsed_id)
}

fn create_rounds_from_rounds_list_str(rounds_list_str: &str) -> Result<Vec<Round>, &'static str> {
    let rounds_str_list = rounds_list_str.split("; ").collect::<Vec<&str>>();
    if rounds_str_list.is_empty() {
        return Err("Failed to extract individual round strings from rounds string");
    }

    let mut rounds: Vec<Round> = vec![];

    for round_str in rounds_str_list.iter() {
        match Round::from(round_str) {
            Ok(round) => rounds.push(round),
            Err(error) => return Err(error),
        }
    }

    Ok(rounds)
}

fn fold_line(acc: i32, line: &str) -> Result<i32, &'static str> {
    let parts = line.split(':').collect::<Vec<&str>>();
    let game_info = parts.get(0).expect("Couldn't retrieve game info from line");
    let id = get_id_from_game_info(game_info)?;
    let rounds_list_str = parts.get(1).expect("Couldn't retrieve rounds from line");
    let rounds = create_rounds_from_rounds_list_str(rounds_list_str)?;
    let game = Game::new(id, rounds).expect("Couldn't create a new game from line");
    let fewest_power = game.calculate_fewest_power();
    Ok(acc + fewest_power)
}

fn process(input: &str) -> Result<i32, &'static str> {
    input.lines().try_fold(0, fold_line)
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    match output {
        Ok(result) => {
            dbg!(result);
        }
        Err(err) => {
            eprintln!("{err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_passes() {
        let process_result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert!(process_result.is_ok_and(|result| result == 2286 as i32));
    }
}
