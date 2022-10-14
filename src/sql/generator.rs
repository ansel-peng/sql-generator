use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::vec;
use crate::sql::common::{Field, fields_array_to_map, get_fields, get_md_field, get_table};


const TRUE: &str = "true";


#[derive(Debug)]
#[allow(dead_code)]
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
    //generate
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
                    let map = fields_array_to_map(array);
                    self.generate_sql(map, value, &mut file);
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
            let map = fields_array_to_map(array);
            self.generate_sql(map, value, &mut file);
        }
    }

    //generate sql
    fn generate_sql(&self, map: HashMap<String, Vec<String>>, table_name: String, file: &mut File) {
        let drop_exist_line = self.drop_exist_table(&table_name);
        if let Some(str) = drop_exist_line {
            file.write_all(str.as_bytes()).expect("write error!");
            let create_line = format!("\nCREATE TABLE `{}` (", &table_name);
            file.write_all(create_line.as_bytes()).expect("write error!");
        }
        const TAB: &str = "\n\t\t\t\t\t\t";
        let mut primary_key = None;
        for i in 0..map.get(Field::Field.call()).expect("no Field").len() {
            if i != 0 {
                file.write_all(b",").expect("write error!");
            }
            let field = get_md_field(Field::Field, &map, i);
            let field_type = get_md_field(Field::Type, &map, i);
            let not_null = get_md_field(Field::NotNull, &map, i);
            let auto_increment = get_md_field(Field::AutoIncrement, &map, i);
            let comment = get_md_field(Field::Comment, &map, i);
            let default = get_md_field(Field::Default, &map, i);
            let primary = get_md_field(Field::Primary, &map, i);
            let mut field_line;
            if auto_increment == TRUE {
                if not_null == TRUE {
                    field_line = format!("{}`{}` {} {}", TAB, field, field_type, "NOT NULL AUTO_INCREMENT");
                } else {
                    std::fs::remove_file("init.sql").expect("remove err!");
                    panic!("{} is is self increasing, but it defaults to null!", field);
                }
            } else {
                if not_null == TRUE {
                    field_line = format!("{}`{}` {} {}", TAB, field, field_type, "NOT NULL");
                } else {
                    field_line = format!("{}`{}` {} {}", TAB, field, field_type, "DEFAULT NULL");
                }
            }
            if !default.is_empty() {
                field_line = field_line + " DEFAULT '" + default + "'";
            }
            if !comment.is_empty() {
                field_line = field_line + " comment '" + comment + "'";
            } else {
                field_line = field_line;
            }
            if !primary.is_empty() {
                primary_key = Some(field);
            }
            file.write_all(field_line.as_bytes()).expect("write error!");
        }
        if let Some(value) = primary_key {
            let primary_line = format!(",{}PRIMARY KEY (`{}`)", TAB, value);
            file.write_all(primary_line.as_bytes()).expect("write error!");
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

