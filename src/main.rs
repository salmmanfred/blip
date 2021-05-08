use openfile;
use std::env;
use std::time::Instant;

mod inter;

fn main() {
    let send: Vec<String> = env::args().collect();
    
    let x = &send[1];
    if x == "-h" || x == "-help"{
        println!("help text goes here");

    }else{
        let file = openfile::readFileLines(&x);
        let now = Instant::now();
        inter::run(file);
        println!("Extime: {}",now.elapsed().as_millis());
    }

}
