use crate::cube::{collection::Collection, color::Color};

#[derive(Debug)]
pub struct Round {
    collections: Vec<Collection>,
}

impl Round {
    pub fn from(cube_color_counts_str: &str) -> Result<Round, &'static str> {
        let mut collections: Vec<Collection> = vec![];
        for color_count_str in cube_color_counts_str.split(", ") {
            let color_count_parts = color_count_str.split_whitespace().collect::<Vec<&str>>();
            let cube_color_str = color_count_parts
                .get(1)
                .expect("Failed to retrieve color string from color count string");
            let cube_color = Color::from(cube_color_str)
                .expect("Failed to retrieve color variant from color count string");
            let cube_count_str = color_count_parts
                .get(0)
                .expect("Failed to retrieve count from color count string");
            let cube_count = cube_count_str
                .parse::<i32>()
                .expect("Failed to parse string count into int");
            collections.push(Collection::new(cube_color, cube_count))
        }
        Ok(Round { collections })
    }

    pub fn has_color_count_exceeding_threshold(&self) -> bool {
        for collection in self.collections.iter() {
            if collection.has_exceeded_color_threshold() {
                return true;
            }
        }
        false
    }
}
