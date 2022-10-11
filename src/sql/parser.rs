use std::any::{Any, TypeId};
use lazy_static::*;
use regex::Regex;

#[derive(Debug)]
pub struct Table {
    table_name: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    name: String,
    field_type: String,
    comment: String,
    primary: bool,
    auto_increment: bool,
    not_null: bool,
    unique: bool,
    default: String,
}

impl Table {
    pub fn new(table_name: String, fields: Vec<Field>) -> Table {
        Table {
            table_name,
            fields,
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            name: String::from(""),
            field_type: String::from(""),
            comment: String::from(""),
            primary: false,
            auto_increment: false,
            not_null: false,
            unique: false,
            default: String::from(""),
        }
    }
}

impl Field {
    pub fn new(name: String, field_type: String, comment: String) -> Field {
        Field {
            name,
            field_type,
            comment,
            ..Default::default()
        }
    }
}

//get table
pub fn get_table(line: &String) -> Option<Table> {
    lazy_static! {
        static ref TABLE_REGEX:Regex = Regex::new(r"^#{3}").unwrap();
    }
    let mut table_name: Option<String> = None;
    if TABLE_REGEX.is_match(line) {
        let mut temp = &line[3..line.len()];
        table_name = Some(temp.trim().parse().unwrap());
    };

    if let Some(value) = table_name {
        let table = Table::new(value, vec![]);
        return Some(table);
    }
    return None;
}