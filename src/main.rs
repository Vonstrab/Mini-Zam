use std::thread;
use std::time::Duration;

use std::io;

mod machine;
mod mlvalue;
mod parser;

fn main() {
    let mut arguments = std::env::args().collect::<Vec<String>>();

    let mut code = parser::parse_prog(parser::reader(arguments[1].as_str()));


    println!("Code de {} :", arguments[1]);
    for inst in &code {
        println!("{:?}", inst);
    }

    let mut machine = machine::ZAM::new(&code);
    machine.set_option("Debug");
    // machine.set_option("Step");
    machine.run();
}
