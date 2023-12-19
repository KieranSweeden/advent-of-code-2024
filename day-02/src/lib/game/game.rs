use crate::game::round::Round;

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub rounds: Vec<Round>,
}

impl Game {
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
}
