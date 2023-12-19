use crate::game::round::Round;

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub rounds: Vec<Round>,
}

impl Game {
    fn get_fewest_color_counts(&self) -> (i32, i32, i32) {
        let mut fewest_red_count = 0;
        let mut fewest_green_count = 0;
        let mut fewest_blue_count = 0;

        for round in self.rounds.iter() {
            let (round_red_count, round_green_count, round_blue_count) = round.get_color_counts();

            if round_red_count > fewest_red_count {
                fewest_red_count = round_red_count;
            }
            if round_green_count > fewest_green_count {
                fewest_green_count = round_green_count;
            }
            if round_blue_count > fewest_blue_count {
                fewest_blue_count = round_blue_count;
            }
        }

        (fewest_red_count, fewest_green_count, fewest_blue_count)
    }

    pub fn new(id: i32, rounds: Vec<Round>) -> Result<Game, &'static str> {
        Ok(Game { id, rounds })
    }

    pub fn is_possible(&self) -> bool {
        for round in self.rounds.iter() {
            if round.has_color_count_exceeding_threshold() {
                return false;
            }
        }

        true
    }

    pub fn calculate_fewest_power(&self) -> i32 {
        let (fewest_red_count, fewest_green_count, fewest_blue_count) =
            self.get_fewest_color_counts();
        fewest_red_count * fewest_green_count * fewest_blue_count
    }
}
