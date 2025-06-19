pub struct Style {
    rgb: bool,
    bold: bool,
    italic: bool,
    underline: bool,
    fg: Option<Colour>,
    bg: Option<Colour>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            rgb: false,
            bold: false,
            italic: false,
            underline: false,
            fg: None,
            bg: None,
        }
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn fg(mut self, colour: Colour) -> Self {
        self.fg = Some(colour);
        self
    }

    pub fn bg(mut self, colour: Colour) -> Self {
        self.bg = Some(colour);
        self
    }

    pub fn paint(&self, input: String) -> String {
        let mut buf = String::from("\x1b[");

        if self.bold {
            buf += ";1"
        };

        if self.italic {
            buf += ";3"
        };

        if self.underline {
            buf += ";4"
        };

        if let Some(c) = &self.fg {
            if self.rgb {
                let (r, g, b) = c.code();
                buf += &format!(";38;2;{};{};{}", r, g, b);
            } else {
                buf += &format!(";38;5;{}", c.code8bit())
            }
        }

        if let Some(c) = &self.bg {
            if self.rgb {
                let (r, g, b) = c.code();
                buf += &format!(";48;2;{};{};{}", r, g, b);
            } else {
                buf += &format!(";38;2;{}", c.code8bit())
            }
        }

        buf += &format!("m{}\x1b[0m", input);
        return buf;
    }
}

pub enum Colour {
    Red,
    Blue,
    Green,
    Cyan,
    White,
    Black,
}

impl Colour {
    pub fn code(&self) -> (u8, u8, u8) {
        match self {
            Self::Red => (255, 0, 0),
            Self::Blue => (0, 255, 0),
            Self::Green => (0, 0, 255),
            Self::Cyan => (0, 255, 255),
            Self::White => (255, 255, 255),
            Self::Black => (0, 0, 0),
        }
    }

    pub fn code8bit(&self) -> u8 {
        match self {
            Self::Red => 1,
            Self::Blue => 4,
            Self::Green => 2,
            Self::Cyan => 6,
            Self::White => 15,
            Self::Black => 0,
        }
    }
}
