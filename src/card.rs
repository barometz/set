#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Green,
    Purple,
    Red,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Count {
    One,
    Two,
    Three,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Shape {
    Oval,
    Wave,
    Diamond,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Fill {
    Open,
    Dashed,
    Full,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub color: Color,
    pub count: Count,
    pub shape: Shape,
    pub fill: Fill,
}

impl Card {
    pub fn new(color: Color, count: Count, shape: Shape, fill: Fill) -> Card {
        Card { color, count, shape, fill }
    }
}
