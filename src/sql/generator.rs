use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::vec;
use crate::sql::common::{Field, fields_array_to_map, get_fields, get_table};


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
        for i in 0..map.get(Field::Field.call()).expect("no Field").len() {
            let field = map.get(Field::Field.call())
                .expect("no Field")
                .get(i)
                .expect("no index!");
            let field_type = map.get(Field::Type.call())
                .expect("no type")
                .get(i)
                .expect("no index!");
            let not_null = map.get(Field::NotNull.call())
                .expect("no not_null")
                .get(i)
                .expect("no index!");
            let auto_increment = map.get(Field::AutoIncrement.call())
                .expect("no auto_increment")
                .get(i)
                .expect("no index!");
            let comment = map.get(Field::Comment.call())
                .expect("no comment")
                .get(i)
                .expect("no index!");
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
            if !comment.is_empty() {
                field_line = field_line + " comment '" + comment + "',";
            } else {
                field_line = field_line + ",";
            }
            file.write_all(field_line.as_bytes()).expect("write error!");
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

