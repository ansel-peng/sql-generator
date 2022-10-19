use std::env;
use crate::sql::generator::Generator;

mod sql;

fn main() {
    let arg = env::args().nth(1).expect("no path given");
    let generator = Generator::default();
    generator.generate(arg);
}





