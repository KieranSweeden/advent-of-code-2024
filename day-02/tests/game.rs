use day_02::game::game::Game;
use day_02::game::round::Round;

#[test]
fn game_create_works() {
    let rounds = vec![
        Round::from("3 blue, 4 red").unwrap(),
        Round::from("1 red, 2 green, 6 blue").unwrap(),
        Round::from("2 green").unwrap(),
    ];

    let result = Game::new(1, rounds);
    assert!(result.is_ok(), "Expected Ok, but got {:?}", result);
}

#[test]
fn game_is_possible_works() {
    let rounds = vec![
        Round::from("3 blue, 4 red").unwrap(),
        Round::from("1 red, 2 green, 6 blue").unwrap(),
        Round::from("2 green").unwrap(),
    ];

    let result = Game::new(1, rounds);
    assert!(result.is_ok_and(|game| game.is_possible()));
}
