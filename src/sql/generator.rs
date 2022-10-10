use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use crate::sql::parser;
use crate::sql::parser::get_sql_line;

#[derive(Debug)]
pub enum Engine {
    MyISAM,
    InnoDB,
}

impl Engine {
    //return engine string
    pub fn call(&self) -> String {
        return match self {
            Engine::MyISAM => { String::from("MyISAM") }
            Engine::InnoDB => { String::from("InnoDB") }
        };
    }
}

pub struct Generator {
    pub charset: String,
    pub engine: Engine,
    pub drop_exist: bool,
}

impl Generator {
    // drop existing table
    pub fn drop_exist_table(&self, table_name: &str) -> Option<String> {
        if self.drop_exist {
            return Some(format!("DROP TABLE IF EXISTS `{}`", table_name));
        }
        return None;
    }

    //generate sql
    pub fn generate(&self, url: String) {
        // open markdown file
        let mut file = OpenOptions::new()
            .read(true)
            .open(url)
            .expect("cannot open file");
        let mut fin = BufReader::new(file);
        let path = Path::new("init.sql");
        let display = path.display();
        // create sql file
        let mut file = match File::create(&path) {
            Err(err) => panic!("couldn't create {}: {:?}", display, err),
            Ok(file) => file,
        };
        for line in fin.lines() {
            let str = get_sql_line(line.as_ref().unwrap());
            println!("{}", line.unwrap());
        }
    }
}

