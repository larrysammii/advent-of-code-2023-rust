#[derive(Debug, Default)]
struct Turn {
    red: u32,
    green: u32,
    blue: u32,
}

impl Turn {
    fn validate(
        &self,
        red: u32,
        green: u32,
        blue: u32,
    ) -> bool {
        self.red <= red
            && self.green <= green
            && self.blue <= blue
    }
}

#[derive(Debug, Default)]
struct Game {
    rounds: Vec<Vec<Turn>>,
}

// Break input to lines
fn parse(input: &str) -> Vec<Vec<Turn>> {
    let mut whole_game = Game::default();
    let lines = input.lines(); // The whole fucking thing.

    for line in lines {
        // Each line into index 0 = "Game", index 1 =
        // "whatever bullshit behind"
        // games = index 1, we only need games.
        let (_, games) = line.split_once(": ").unwrap();

        // Take the fucking "8 red, 2 green, 3 blue;
        // 69 red, 11 green, 96 blue;..."
        // and break it into chunks like "8 red, 2 green,
        // 3 blue", "69 red, 11 green, 96 blue",
        // "..."
        let games = games.split("; ").collect::<Vec<_>>();

        let mut game_list = Vec::new();
        // FOR EACH FUCKING CHUNK OF GAME
        for g in games {
            // FUCKING SPLIT IT INTO RGB THEN COLLECT IT
            // INTO LIST "69 red", "11 green",
            // "96 blue"
            let cubes = g.split(", ").collect::<Vec<_>>();
            let mut round = Turn::default();
            // EACH FUCKING SINGLE CUBE
            for c in cubes {
                // SPLIT IT WITH THE FUCKING BLANK SPACE
                // amount IS THE FIRST SPLITTED CRAP
                // color IS THE SECOND OF THAT
                let (amount, color) =
                    c.split_once(" ").unwrap();

                let amount = amount.parse::<u32>().unwrap();

                // INSTANTIATE A FUCKING ROUND STRUCT

                // THEN FUCKING MAP IT
                match color {
                    "red" => round.red = amount,
                    "green" => round.green = amount,
                    "blue" => round.blue = amount,
                    _ => panic!("Fuck."),
                }
            }
            game_list.push(round);
        }
        whole_game.rounds.push(game_list);
    }

    whole_game.rounds
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let mut valid_games_sum: u32 = 0;

    let game = parse(input);

    'next: for (i, turns) in game.iter().enumerate() {
        for turn in turns {
            if !turn.validate(12, 13, 14) {
                continue 'next;
            }
        }
        valid_games_sum += i as u32 + 1;
    }
    Ok(valid_games_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        // println!("{:?}", parse(input));
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
