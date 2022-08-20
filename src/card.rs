use std::fmt::Display;

use colored::Colorize;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Green,
    Purple,
    Red,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Count {
    One,
    Two,
    Three,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Shape {
    Oval,
    Wave,
    Diamond,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Fill {
    Open,
    Dashed,
    Full,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Card {
    pub color: Color,
    pub count: Count,
    pub shape: Shape,
    pub fill: Fill,
}

impl Card {
    pub fn new(color: Color, count: Count, shape: Shape, fill: Fill) -> Card {
        Card {
            color,
            count,
            shape,
            fill,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        text += match self.shape {
            Shape::Oval => "💊",
            Shape::Wave => "🌊",
            Shape::Diamond => "🔶",
        };

        text += match self.count {
            Count::One => "1",
            Count::Two => "2",
            Count::Three => "3",
        };

        text += match self.fill {
            Fill::Open => "O",
            Fill::Dashed => "D",
            Fill::Full => "F",
        };

        match self.color {
            Color::Green => f.write_fmt(format_args!("{}", text.green())),
            Color::Purple => f.write_fmt(format_args!("{}", text.purple())),
            Color::Red => f.write_fmt(format_args!("{}", text.red())),
        }
    }
}
