use std::borrow::Borrow;
use lazy_static::*;
use regex::Regex;

pub fn get_sql_line(line: &String) -> Option<String> {
    lazy_static! {
        static ref TABLE_REGEX:Regex = Regex::new(r"^#{3}").unwrap();
    }
    let mut table_name: Option<String> = None;
    if TABLE_REGEX.is_match(line) {
        let mut temp = &line[3..line.len()];
        table_name = Some(temp.trim().parse().unwrap());
    };
    if let Some(value) = table_name {
        return Some(format!("create table `{}` (", value));
    }
    return None;
}