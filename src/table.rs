use amlich::{dow::ShortTitle, Calendar, DayOfWeek, GregorianMonth};
use std::any::Any;
use std::convert::TryInto;
use std::fmt::{Display, Error, Formatter};
use std::io::Read;
use std::time::Duration;
use tui::backend::RustboxBackend;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::Terminal;

pub struct CalTable {
    month: GregorianMonth,
    terminal: Terminal<RustboxBackend>,
}

fn get_current_month() -> GregorianMonth {
    GregorianMonth::new(2019, 9)
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

const SLEEP_INTERVAL: Duration = Duration::from_millis(10);

fn read_key() -> Result<u8, std::io::Error> {
    let mut buf = [0u8, 1];
    while std::io::stdin().read(&mut buf)? == 0 {
        std::thread::sleep(SLEEP_INTERVAL);
    }
    Ok(buf[0])
}

impl CalTable {
    pub fn new() -> CalTable {
        let terminal = Terminal::new(RustboxBackend::new().unwrap()).unwrap();
        CalTable {
            month: get_current_month(),
            terminal,
        }
    }
    pub fn run(&mut self) {
        let mut s = String::from("AHHI");
        loop {
            let mut table = get_table();
            self.terminal.draw(|mut f| {
                let size = f.size();
                table.render(&mut f, size);
            });
            read_key().unwrap();
            break;
        }
    }
}
