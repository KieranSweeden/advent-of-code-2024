use day_02::cube::collection::Collection;
use day_02::cube::color::Color;

#[test]
fn valid_color_str_returns_some_color_variant() {
    let red_cube_variant = Color::from("red");
    let green_cube_variant = Color::from("green");
    let blue_cube_variant = Color::from("Blue");

    assert!(matches!(red_cube_variant, Some(Color::Red)));
    assert!(matches!(green_cube_variant, Some(Color::Green)));
    assert!(matches!(blue_cube_variant, Some(Color::Blue)));
}

#[test]
fn invalid_color_str_returns_none() {
    let invalid_variant = Color::from("re");
    assert!(matches!(invalid_variant, None));
}

#[test]
fn collection_create_works() {
    let count_within_threshold = 5;
    let collection = Collection::new(Color::Red, count_within_threshold);
    assert!(matches!(collection.color, Color::Red));
    assert_eq!(collection.count, count_within_threshold);
}

#[test]
fn collection_exceeding_threshold_works() {
    let count_exceeding_threshold = 15;
    let collection = Collection::new(Color::Red, count_exceeding_threshold);
    assert_eq!(collection.has_exceeded_color_threshold(), true);
}
