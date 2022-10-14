use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Field {
    Field,
    Type,
    Comment,
    Primary,
    AutoIncrement,
    NotNull,
    Unique,
    Default,
}


impl Field {
    pub fn call(&self) -> &str {
        return match self {
            Field::Field => { "field" }
            Field::Type => { "type" }
            Field::Comment => { "comment" }
            Field::Primary => { "primary" }
            Field::AutoIncrement => { "auto_increment" }
            Field::NotNull => { "not_null" }
            Field::Unique => { "unique" }
            Field::Default => { "default" }
        };
    }
}


//get table
pub fn get_table(line: &str) -> Option<String> {
    lazy_static! {
        static ref TABLE_REGEX:Regex = Regex::new(r"^#{3}").unwrap();
    }
    let table_name: String;
    if TABLE_REGEX.is_match(line) {
        let temp = &line[3..line.len()];
        table_name = temp.trim().parse().unwrap();
        if table_name.is_empty() {
            return None;
        }
        return Some(String::from(table_name));
    };
    return None;
}

//get fields
pub fn get_fields(line: &str, array: &mut [Vec<String>; 8]) {
    let line = &line[1..=line.len() - 2];
    let fields: Vec<&str> = line.split("|").collect();
    if fields.len() == 8 {
        let mut i = 0;
        for field in fields {
            array[i].push(String::from(field.trim()));
            i += 1;
        }
    }
}

pub fn fields_array_to_map(mut array: [Vec<String>; 8]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for i in 0..array.len() {
        let key = array[i].get(0).expect("no fields").to_string();
        let mut vec = vec![];
        //delete one line and two line
        array[i].remove(0);
        array[i].remove(0);
        for value in &array[i] {
            vec.push(value.to_string());
        }
        map.insert(key.to_lowercase(), vec);
    }
    return map;
}