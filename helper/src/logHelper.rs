use ansi_term::Color::{Green, Red, Yellow};
use log::*;
use chrono::prelude::*;

pub fn Info(info: String) -> String {
    return Green.bold().paint(format!("{}", info)).to_string();

}

pub fn printLn(info: String, target: String) -> String {
    let local: DateTime<Local> = Local::now();
    return Green.bold().paint(format!(
        "< {} {} {} {} >",
        local,
        "INFO",
        info,
        target)
    ).to_string();
}

pub fn Error(err: String, target: String) -> String {
    let local: DateTime<Local> = Local::now();
    return Red.bold().paint(format!(
        "< {} {} {} {} >",
        local,
        "ERROR",
        err,
        target)
    ).to_string();
}


pub fn Warn(warm: String, target: String) -> String {
    let local: DateTime<Local> = Local::now();
    return Yellow.bold().paint(format!(
        "< {} {} {} {} >",
        local,
        "WARN",
        warm,
        target)
    ).to_string();
}

pub fn Debug(warm: String) -> String {
    return Yellow.bold().paint(format!("{}", warm)).to_string();
}

pub fn Trace(warm: String) -> String {
    return Yellow.bold().paint(format!("{}", warm)).to_string();
}