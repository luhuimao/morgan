use ansi_term::Color::{Green, Red, Yellow, Blue, Cyan, White};
use log::*;
use chrono::prelude::*;

pub fn Info(info: String) -> String {
    return Green.bold().paint(format!("{}", info)).to_string();

}

pub fn printLn(info: String, target: String) -> String {
    let local: DateTime<Local> = Local::now();
    return format!(
        "{} {} {} {} {} {}",
        Cyan.bold().paint(format!("<")),
        White.bold().paint(format!("{}", local)),
        Green.bold().paint(format!("INFO")),
        Green.bold().paint(format!("{}", info)),
        Blue.bold().paint(format!("{}", target)),
        Cyan.bold().paint(format!(">"))
    ).to_string();
}

pub fn Error(err: String, target: String) -> String {
    let local: DateTime<Local> = Local::now();
    return format!(
        "{} {} {} {} {} {}",
        Cyan.bold().paint(format!("<")),
        White.bold().paint(format!("{}", local)),
        Red.bold().paint(format!("ERROR")),
        Red.bold().paint(format!("{}", err)),
        Blue.bold().paint(format!("{}", target)),
        Cyan.bold().paint(format!(">"))
    ).to_string();
}


pub fn Warn(warn: String, target: String) -> String {
    let local: DateTime<Local> = Local::now();
    return format!(
        "{} {} {} {} {} {}",
        Cyan.bold().paint(format!("<")),
        White.bold().paint(format!("{}", local)),
        Yellow.bold().paint(format!("WARN")),
        Yellow.bold().paint(format!("{}", warn)),
        Blue.bold().paint(format!("{}", target)),
        Cyan.bold().paint(format!(">"))
    ).to_string();
}

pub fn Debug(debug: String, target: String) -> String {
    let local: DateTime<Local> = Local::now();
    return format!(
        "{} {} {} {} {} {}",
        Cyan.bold().paint(format!("<")),
        White.bold().paint(format!("{}", local)),
        Yellow.bold().paint(format!("WARN")),
        Yellow.bold().paint(format!("{}", debug)),
        Blue.bold().paint(format!("{}", target)),
        Cyan.bold().paint(format!(">"))
    ).to_string();
}

pub fn Trace(trace: String, target: String) -> String {
    let local: DateTime<Local> = Local::now();
    return format!(
        "{} {} {} {} {} {}",
        Cyan.bold().paint(format!("<")),
        White.bold().paint(format!("{}", local)),
        Yellow.bold().paint(format!("WARN")),
        Yellow.bold().paint(format!("{}", trace)),
        Blue.bold().paint(format!("{}", target)),
        Cyan.bold().paint(format!(">"))
    ).to_string();
}