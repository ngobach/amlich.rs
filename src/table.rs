use amlich::{dow::ShortTitle, Calendar, DayOfWeek, GregorianMonth};
use std::convert::TryInto;
use std::fmt::{Display, Error, Formatter};
use termion::{clear, color, cursor, style};

pub struct Table {
    inner: GregorianMonth,
}

fn too_small_digits(s: &str) -> String {
    fn char_2_char(c: char) -> char {
        match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => panic!("Oops"),
        }
    }
    let mut ss = String::with_capacity(s.len());
    for i in s.chars() {
        ss.push(char_2_char(i));
    }
    ss
}

impl Table {
    const CELL_WIDTH: u8 = 3;
    const CELL_HEIGHT: u8 = 2;

    pub fn of(month: GregorianMonth) -> Self {
        Self { inner: month }
    }

    fn render(&self) -> String {
        let mut buffer = format!("{}", clear::All);
        buffer.push_str(color::Green.fg_str());
        let cursor_down = format!("{}", cursor::Down(1));
        let cursor_up = format!("{}", cursor::Up(1));
        for i in 0..7 {
            let day: DayOfWeek = i.try_into().unwrap();
            buffer.push_str(day.short_title().as_str());
            buffer.push(' ');
        }
        buffer.push_str(color::Reset.fg_str());
        buffer.push_str("\n\n\n");
        let bound = self.inner.get_bound();
        let blank = bound.iter().next().unwrap().day_of_week() as u32;
        for i in 0..blank {
            buffer.push_str(
                format!(
                    "{}   {}    ",
                    format!("{}", cursor::Up(1)),
                    format!("{}{}", cursor::Down(1), cursor::Left(3)),
                )
                .as_str(),
            );
        }
        for day in bound.iter() {
            if day.day_of_week() == DayOfWeek::Sunday {
                buffer.push_str("\n\n\n");
            }

            buffer.push_str(
                format!(
                    "{}{:3}{}{:3} ",
                    format!("{}", cursor::Up(1)),
                    day.inner.day,
                    format!("{}{}", cursor::Down(1), cursor::Left(3)),
                    too_small_digits(format!("{}", day.to_lunar().inner.day).as_str()),
                )
                .as_str(),
            );
        }
        buffer
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(self.render().as_str())
    }
}
