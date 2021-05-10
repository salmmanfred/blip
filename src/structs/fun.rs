use crate::inter;
use crate::vars::var;
use crate::small;
pub struct fun {
    pub names: Vec<String>,
    pub n_arg: Vec<Vec<String>>,
    pub start: Vec<i64>,
    pub end: Vec<i64>,
}
impl fun {
    // find all the functions and put them in the struct
    pub fn find_fun(&mut self, text: Vec<String>) {
        let mut find = true;
        for x in 0..text.len() {
            let mut split = text[x].split(" ").collect::<Vec<&str>>();
            if find {
                if split[0] == "FN" {
                    self.names.push(split[1].to_string());
                    self.start.push(x as i64);
                    find = false;
                    split.remove(0);
                    split.remove(0);
                    

                    let mut split: Vec<String> = split.iter().map(|x|x.to_string()).collect();
                    //split.retain(|x| x != "|");
                    self.n_arg.push(split);
                }
            } else {
                if split[0] == "STOP_FN" {
                    self.end.push(x as i64);
                    find = true;
                }
            }
        }
    }
    // get the last row of the function
    pub fn get_last(&self, name: &str) -> usize {
        for x in 0..self.names.len() {
            if self.names[x] == name {
                return self.end[x] as usize;
            }
        }
        panic!("No function with that name");
    }
    // get the first row of the function
    pub fn get_first(&self, name: &str) -> usize {
        for x in 0..self.names.len() {
            if self.names[x] == name {
                return self.start[x] as usize;
            }
        }
        panic!("No function with that name");
    }
    pub fn ppos(&self, name: &str)->usize{
        for x in 0..self.names.len() {
            if self.names[x] == name {
                return x;
            }
        }
        panic!("No function with that name");
    }
    // run a function
    pub fn run(&mut self, name: &str, text: Vec<String>, vars: &mut var,args: Vec<String>) {
        let xx = self.ppos(name);
        for x in 0..self.n_arg[xx].len(){
            if self.n_arg[xx][x] == "I".to_string(){
                vars.new_var_i(&x.to_string(),&small::get_value(args[x].clone(),vars.clone()));
            }
            if self.n_arg[xx][x] == "S".to_string(){
                
                vars.new_var_string(&x.to_string(),&small::get_value(args[x].clone(),vars.clone()));

            }
        }
        inter::parse_run(
            [
                self.get_first(name) as i64 + 1 as i64,
                self.get_last(name) as i64,
            ],
            text,
            vars,
            self,
        );

    }
    pub fn run_loop(&mut self, name: &str, text: Vec<String>, vars: &mut var, max_laps: i64,args: Vec<String>) {
        let mut laps = 0;
        let xx = self.ppos(name);
        
        while laps <= max_laps {
            for x in 0..self.n_arg[xx].len(){
                if self.n_arg[xx][x] == "I".to_string(){

                    vars.new_var_i(&x.to_string(),&small::get_value(args[x].clone(),vars.clone()));

                }
                if self.n_arg[xx][x] == "S".to_string(){
                    
                    vars.new_var_string(&x.to_string(),&small::get_value(args[x].clone(),vars.clone()));

    
                }
            }
            let text = text.clone();
            inter::parse_run(
                [
                    self.get_first(name) as i64 + 1 as i64,
                    self.get_last(name) as i64,
                ],
                text,
                vars,
                self,
            );
            laps += 1;
            if max_laps == 0 {
                laps -= 1;
            }
        }
    }

    pub fn new() -> fun {
        fun {
            names: Vec::new(),
            start: Vec::new(),
            n_arg: Vec::new(),
            end: Vec::new(),
        }
    }
}
