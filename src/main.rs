mod symbols;
use chrono::Local;
use clap::Parser;
use colors_transform::{Color, Rgb};
use core::panic;
use std::io::{self, Write};
use std::process::{exit, ExitCode};
use std::sync::mpsc;
use std::time::Duration;
use std::time::Instant;
use std::{char, thread, u16};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{IntoAlternateScreen, ToMainScreen};
use termion::terminal_size;
use termion::{clear, color};
#[derive(Parser)]
#[command(version, about="cli clock/stopwacth",author="Archerymystery", long_about = None)]
struct Cli {
    /// Center a clock
    #[arg(short)]
    center: bool,

    /// Stopwatch mode
    #[arg(short = 'S')]
    stopwatch: bool,

    /// Display in 12 hour clock format
    #[arg(short)]
    r: bool,

    /// Display seconds
    #[arg(short)]
    seconds: bool,

    /// Change char in the clock
    #[arg(short = 'C', long, default_value = "█", value_name = "CHAR")]
    char: char,

    /// Change clock color
    #[arg(short = 'H', long, default_value = "#FFFFFF", value_name = "HEX")]
    hex: String,

    /// Date format
    #[arg(short = 'F', long, default_value = None)]
    format: Option<String>,
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let stdin = io::stdin();
    let stdout = io::stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    let mut stdout = stdout.lock();
    let symbols_list: [[[&str; 3]; 5]; 11] = symbols::get_symbols();
    let (tx, rx) = mpsc::channel();
    let rgb = Rgb::from_hex_str(cli.hex.as_str());
    let rgb = match rgb {
        Ok(_) => rgb,
        Err(e) => {
            write!(stdout, "{}{}{e:?}", ToMainScreen, color::Fg(color::Red)).unwrap();
            stdout.flush().unwrap();
            exit(103);
        }
    };
    let rgb_unwrap = rgb.unwrap();
    let color_rgb = color::Rgb(
        rgb_unwrap.get_red() as u8,
        rgb_unwrap.get_green() as u8,
        rgb_unwrap.get_blue() as u8,
    );

    let start = Instant::now();
    thread::spawn(move || {
        for key in stdin.keys() {
            match key {
                Ok(Key::Char(c)) => {
                    if tx.send(c).is_err() {
                        break;
                    }
                }
                _ => {}
            }
        }
    });

    loop {
        let unix_time = Local::now();
        let mut time_format: String = "%H:%M".to_string();
        if cli.r {
            time_format = "%I:%M".to_string();
        }
        if cli.seconds {
            time_format.push_str(":%S")
        }

        let mut time = unix_time.format(&time_format).to_string();
        let mut date_format = "%d.%m.%Y".to_string();
        if cli.stopwatch {
            date_format.push_str(" ");
            date_format.push_str(time_format.as_str());
            time = format_duration(start.elapsed());
        }
        if cli.r {
            date_format.push_str(" [%p]");
        }

        let date = match cli.format {
            None => unix_time.format(&date_format).to_string(),
            _ => unix_time
                .format(&cli.format.clone().expect("REASON"))
                .to_string(),
        };
        let mut time_id_list: Vec<i32> = Vec::with_capacity(time.len());
        for symbol in time.chars() {
            let id: i32 = match symbol {
                ':' => 10,
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    symbol.to_digit(10).unwrap().try_into().unwrap()
                }
                _ => {
                    panic!("An unexpected symbol \'{symbol}\' in \"{time}\"");
                }
            };
            time_id_list.push(id);
        }
        let count_colon: i32 = time_id_list
            .iter()
            .filter(|&&x| x == 10)
            .count()
            .try_into()
            .unwrap();
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        let length = count_colon * 3 + (time_id_list.len() as i32 - count_colon) * 7 - 1;
        let mut retreat_x = 1;
        let mut retreat_y = 1;
        let mut retreat_date_x = length as u16 / 2 - date.len() as u16 / 2;
        let mut retreat_date_y = 7;
        if cli.center {
            let (x, y) = terminal_size().unwrap();
            retreat_x = x / 2 - (length as u16) / 2;
            retreat_y = y / 2 - 2;
            retreat_date_x = x / 2 - date.len() as u16 / 2;
            retreat_date_y = y / 2 + 4;
        }
        for line_id in 0..5 {
            let mut line: String = String::new();
            for id in &time_id_list {
                let id_usize: usize = *id as usize;
                for symbol in symbols_list[id_usize][line_id] {
                    line.push_str(symbol);
                }
                line.push_str(" ")
            }
            let buf = cli.char.to_string();
            let char_str = buf.as_str();
            write!(
                stdout,
                "{}{}{}",
                cursor::Goto(retreat_x, (retreat_y + line_id as u16).try_into().unwrap()),
                color::Fg(color_rgb),
                line.replace("█", char_str),
            )
            .unwrap();
        }
        write!(
            stdout,
            "{}{}{}",
            color::Fg(color_rgb),
            cursor::Goto(retreat_date_x, retreat_date_y),
            date
        )
        .unwrap();
        stdout.flush().unwrap();

        if let Ok(c) = rx.try_recv() {
            match c {
                'q' | 'Q' => {
                    write!(stdout, "{}{}", ToMainScreen, cursor::Show).unwrap();
                    stdout.flush().unwrap();
                    exit(0)
                }
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
