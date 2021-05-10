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

fn main() {
    let send: Vec<String> = env::args().collect();

    let x = &send[1];
    if x == "-h" || x == "-help" {
        println!("If you consider using this here are some emergency hotlines you should call: 911, 112, 999");
    } else {
        let mut file = openfile::readFileLines(&x);
        file.retain(|x| x != "");

        let now = Instant::now();
        inter::run(file);
        println!("Extime: {}", now.elapsed().as_millis());
    }
}
