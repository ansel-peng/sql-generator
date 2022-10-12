use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::vec;
use lazy_static::lazy_static;
use regex::Regex;


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

#[derive(Debug)]
pub struct Generator<'a> {
    charset: &'a str,
    engine: Engine,
    drop_exist: bool,
}

impl Default for Generator<'_> {
    fn default() -> Self {
        Generator {
            charset: "utf8",
            engine: Engine::InnoDB,
            drop_exist: true,
        }
    }
}

impl Generator<'_> {
    //generate sql
    pub fn generate(&self, url: String) {
        // open markdown file
        let file = OpenOptions::new()
            .read(true)
            .open(url)
            .expect("cannot open file");
        let fin = BufReader::new(file);
        let path = Path::new("init.sql");
        let display = path.display();
        // create sql file
        let mut file = match File::create(&path) {
            Err(err) => panic!("couldn't create {}: {:?}", display, err),
            Ok(file) => file,
        };
        file.write_all(b"-- init sql").expect("write error");
        let mut table = None;
        //Identifies the first match to the table name
        let mut flag = false;
        let mut array = core::array::from_fn(|_| vec![]);
        for line in fin.lines() {
            //get table
            let table_option = get_table(line.as_ref().unwrap().trim());
            if let Some(value) = table_option {
                if let Some(value) = table {
                    self.generate_sql(array, value, &mut file);
                    array = core::array::from_fn(|_| vec![]);
                }
                table = Some(value);
                flag = true;
                continue;
            }
            //get fields
            let line = line.as_ref().unwrap().trim();
            if !line.is_empty() && flag {
                get_fields(line, &mut array);
            }
        }
        if let Some(value) = table {
            self.generate_sql(array, value, &mut file);
        }
    }

    fn generate_sql(&self, array: [Vec<String>; 8], table_name: String, file: &mut File) {
        let drop_exist_line = self.drop_exist_table(&table_name);
        if let Some(str) = drop_exist_line {
            file.write_all(str.as_bytes()).expect("write error!");
            let create_line = format!("\nCREATE TABLE `{}` (", &table_name);
            file.write_all(create_line.as_bytes()).expect("write error!");
        }
        const TAB: &str = "\n\t\t\t\t\t\t";
        for i in &array[0] {
            
        }
        let end_line = format!("\n) ENGINE={} DEFAULT CHARSET={};", self.engine.call(), self.charset);
        file.write_all(end_line.as_bytes()).expect("write error!");
    }

    // drop existing table
    fn drop_exist_table(&self, table_name: &String) -> Option<String> {
        if self.drop_exist {
            return Some(format!("\nDROP TABLE IF EXISTS `{}`;", table_name));
        }
        return None;
    }
}

//get table
fn get_table(line: &str) -> Option<String> {
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
fn get_fields(line: &str, array: &mut [Vec<String>; 8]) {
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

