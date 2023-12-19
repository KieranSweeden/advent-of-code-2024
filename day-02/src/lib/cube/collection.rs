use crate::cube::color::Color;

#[derive(Debug)]
pub struct Collection {
    pub color: Color,
    pub count: i32,
}

impl Collection {
    pub fn new(color: Color, count: i32) -> Collection {
        Collection { color, count }
    }

    pub fn has_exceeded_color_threshold(&self) -> bool {
        self.count > self.color.threshold()
    }
}
