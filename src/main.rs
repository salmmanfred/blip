use openfile;
use std::env;
use std::time::Instant;

mod inter;

fn main() {
    let send: Vec<String> = env::args().collect();
    
    let x = &send[1];
    if x == "-h" || x == "-help"{
        println!("If you consider using this here are some emergency hotlines you should call: 911, 112, 999");

    }else{
        let file = openfile::readFileLines(&x);
        let now = Instant::now();
        inter::run(file);
        println!("Extime: {}",now.elapsed().as_millis());
    }

}
