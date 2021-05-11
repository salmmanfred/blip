use openfile;
use std::env;
use std::time::Instant;
#[path = "structs/fun.rs"]
mod fun;
#[path = "libs/lib.rs"]
mod lib;
#[path = "structs/vars.rs"]
mod vars;

#[path = "small/small.rs"]
mod small;

mod inter;
mod parser;

fn main() {
    let send: Vec<String> = env::args().collect();
    println!("<Blip 2021 GPL-v3.0>");
    let x = &send[1];
    if x == "-h" || x == "-help" {
        println!("If you consider using this here are some emergency hotlines you should call: 911, 112, 999");
    } else {
        let mut file = openfile::readFileLines(&x);
        file.retain(|x| x != "");
        let par = parser::parser(file);
        let now = Instant::now();
        //inter::run(file);
        inter::inter([0, par.parsed_data.len()], par);
        if send.len() > 2 {
            if &send[2] == "-d" || &send[2] == "-debug" {
                println!("Extime: {}", now.elapsed().as_millis());
            }
        }
    }
}
