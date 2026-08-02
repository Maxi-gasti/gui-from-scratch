use crate::models; // to use the models.rs from the crate and not create another module.
use crate::time_weather;
use std::fs;
use std::io::Write;

const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[1;33m";
const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";

pub fn log(log_type: models::Log, title: &str, text: &str){
    
    if let Ok(mut file_log) = fs::OpenOptions::new().create(true).append(true).open("log.txt") {

        match log_type {
            models::Log::Info => {
                let local_time = time_weather::get_time();
                let f_text = format!("{}: {}{}:{} {}",local_time,YELLOW,title,RESET,text);
                writeln!(file_log,"{}",f_text);
            },
            models::Log::Error => {
                let local_time = time_weather::get_time();
                let f_text = format!("{}: {}{}:{} {}",local_time,RED,title,RESET,text);
                writeln!(file_log,"{}",f_text);
            },
            models::Log::Execute => {
                let local_time = time_weather::get_time();
                let f_text = format!("{}: {}{}{}",local_time,GREEN,title,RESET);
                writeln!(file_log,"{}",f_text);
            }
        }
    }
}
