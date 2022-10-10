use std::env;
use crate::sql::generator::{Engine, Generator};

mod sql;

fn main() {
    let arg = env::args().nth(1).expect("no path given");
    let generator = Generator {
        charset: "".to_string(),
        engine: Engine::MyISAM,
        drop_exist: false,
    };
    generator.generate(arg);
}


