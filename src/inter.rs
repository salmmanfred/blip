//use crate::fun::fun;
#[allow(unused_imports)]
use crate::lib;
use crate::vars::Var;
//use std::io::{self};
use crate::parser::{Command, Parse};
use crate::small;
use openfile;
/*pub fn run(text: Vec<String>) {
    let mut vars = var::new();
    let mut funs = fun::new();
    let text = lib::load(text);
    funs.find_fun(text.clone());

    // test vars
    // all variables are public
    vars.new_var_string("LK", "t");
    vars.new_var_string("RK", "t2");
    //run the code
    parse_run([0, text.clone().len() as i64], text, &mut vars, &mut funs);
}

pub fn parse_run(se: [i64; 2], text: Vec<String>, vars: &mut var, funs: &mut fun) {
    let mut i: bool = false;
    let mut modif: i64 = 0;
    for OP in se[0]..se[1] as i64 {
        let OP = (OP + modif) as usize;
        if OP >= text.len() {
            // so that it dose not over run itself
            break;
        }
        let mut split_OP = text[OP].split(" ").collect::<Vec<&str>>();
        if split_OP[0] == "IF" {
            // go into the if i if this is true
            i = true;
        }
        // this runs if its an if
        if i {
            let mut comp1S = "".to_string();
            let mut comp2S = "".to_string();
            let mut moda = 0;
            let mut eqfun = "";
            let mut iftrue = false;
            {
                // get comp1
                let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM" {
                    comp1S = vars.get_var(a[1].to_string().clone());
                } else {
                    comp1S = split_OP[1].to_string().clone();
                }
            }
            {
                //if it should be == or something else
                eqfun = split_OP[2]
            }
            {
                let a = split_OP[3].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM" {
                    comp2S = vars.get_var(a[1].to_string().clone());
                } else {
                    comp2S = split_OP[3].to_string().clone();
                }

                moda = 3;
            }

            // stop if it should stop
            if text[OP] == "STOP" {
                i = false;
            }
            // do the if
            if moda == 3 {
                if eqfun == "==" {
                    if comp1S == comp2S {
                        iftrue = true;
                    }
                }
                if eqfun == "!=" {
                    if comp1S != comp2S {
                        iftrue = true;
                    }
                }
            }

            // if it turns out to be true it runs the things in the if
            if iftrue {
                /*iftrue = false;
                eqfun = "";
                moda = 0;
                comp1S = "".to_string();
                comp2S = "".to_string();*/
                i = false;
                //modif -= 1;
            }
        } else {
            // print
            if split_OP[0] == "PR" {
                // let a = text[OP+1].split(":").collect::<Vec<&str>>();
                let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM" {
                    println!("{}", vars.get_var(a[1].to_string()));
                } else {
                    split_OP.remove(0);
                    let print = split_OP.join(" ");
                    println!("{}", print);
                }
                //modif += 1;
            }
            if split_OP[0] == "GET" {
                // let a = text[OP+1].split(":").collect::<Vec<&str>>();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("ERROR reading the input buffer");


                let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM" {

                    vars.up_var(a[1], &input);
                } else {
                    panic!("You need to add a variable here for the result to be read into")
                }
                //modif += 1;
            }
            // ! variable stuff
            // make a variable
            if split_OP[0] == "MKV:S" {
                vars.new_var_string(&split_OP[1], &split_OP[2]);
                // modif += 2;
            }
            // make a new i variable
            if split_OP[0] == "MKV:I" {
                vars.new_var_i(&split_OP[1], &split_OP[2]);
                // modif += 2;
            }
            // add a number to a number
            if split_OP[0] == "ADD" {
                let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM" {
                    let add = vars.get_var(a[1].to_string()).parse::<i64>().unwrap();

                    let amount = split_OP[2].parse::<i64>().unwrap();
                    let total = add + amount;

                    vars.up_var(a[1], &total.to_string());
                    // modif += 2;
                } else {
                    panic!("WHAT");
                }
            }
            // change the variable so like var = thisnow
            if split_OP[0] == "CHA" {
                let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM" {
                    split_OP.remove(0);
                    split_OP.remove(1);
                    let new = split_OP.join(" ");

                    vars.up_var(a[1], &new);
                    //modif += 2;
                } else {
                    panic!("WHAT");
                }
            }
            if split_OP[0] == "DEL" {
                let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM" {
                    vars.del_var(a[1]);
                    //modif += 2;
                } else {
                    vars.del_var(split_OP[1]);
                }
            }

            // ! function stuff

            // if it detects a fun it jumps over it
            if split_OP[0] == "FN" {
                let x = funs.get_last(split_OP[1]);
                modif += x as i64 - OP as i64;
            }
            // if it finds the run key it runs the function
            if split_OP[0] == "RUN" {
                let x = get_args(text[OP].clone(), false);
                funs.run(split_OP[1], text.clone(), vars, x);
            }
            if split_OP[0] == "LOOP" {
                let mut x_laps = 0;

                x_laps = split_OP[1].parse::<i64>().unwrap();
                let x = get_args(text[OP].clone(), true);

                funs.run_loop(split_OP[2], text.clone(), vars, x_laps, x);
            }
            // ! library stuff
        }
    }
}
fn get_args(string: String, loo: bool) -> Vec<String> {
    let mut s_len = string.split(" ").collect::<Vec<&str>>();
    s_len.remove(0);
    s_len.remove(0);
    if loo {
        s_len.remove(0);
    }
    if s_len.len() < 1 {
        return vec!["".to_string()];
    }
    s_len.iter().map(|x| x.to_string()).collect()
}
*/

pub fn inter(size: [usize; 2], code: Parse) {
    let mut vars = Var::new();
    inter_back(size, code, &mut vars)
}
pub fn inter_back(size: [usize; 2], code: Parse, vars: &mut Var) {
    let mut modif = 0;
    //let mut vars = vars;
    let size_1 = size[0];
    let size_2 = size[1];

    for pos_ in size_1..size_2 {
        let pos_ = pos_ + modif;
        if pos_ >= size_2 {
            break;
        }
        let cur = code.parsed_data[pos_].clone();
        match cur {
            Command::Prints(a) => {
                println!("{}", small::get_value(a, vars.clone()));
            }
            Command::If(pa, if_a) => {
                let a = if_a[0].clone();
                let b = if_a[1].clone();
                let c = if_a[2].clone();

                let mut is_true = false;
                let b = b.as_str();
                match b {
                    "==" => {
                        if small::get_value(a, vars.clone()) == small::get_value(c, vars.clone()) {
                            is_true = true;
                        }
                    }
                    _ => {
                        panic!(format!("This is not a thing: {}, Line: {}", b, pos_))
                    }
                }
                if !is_true {
                    modif += pa[1] as usize - pos_;
                    // println!("mof : {}",modif)
                }
            }
            Command::Fn(_a, b, _c) => {
                modif += b[1] as usize - pos_;
            }
            Command::Run(_a, b, args, arg1) => {
                for x in 0..arg1.len() {
                    if arg1[x] == "I".to_string() {
                        vars.new_var_i(
                            &x.to_string(),
                            &small::get_value(args[x].clone(), vars.clone()),
                        );
                    }
                    if arg1[x] == "S".to_string() {
                        vars.new_var_string(
                            &x.to_string(),
                            &small::get_value(args[x].clone(), vars.clone()),
                        );
                    }
                }
                inter_back([b[0] as usize + 1, b[1] as usize], code.clone(), vars)
            }
            Command::Loop(_a, b, args, arg1, max_laps) => {
                let mut laps = 0;
                while laps <= max_laps {
                    for x in 0..arg1.len() {
                        if arg1[x] == "I".to_string() {
                            vars.new_var_i(
                                &x.to_string(),
                                &small::get_value(args[x].clone(), vars.clone()),
                            );
                        }
                        if arg1[x] == "S".to_string() {
                            vars.new_var_string(
                                &x.to_string(),
                                &small::get_value(args[x].clone(), vars.clone()),
                            );
                        }
                    }

                    inter_back([b[0] as usize + 1, b[1] as usize], code.clone(), vars);
                    laps += 1;
                    if max_laps == 0 {
                        laps -= 1;
                    }
                    //vars.dump()
                }
            }
            Command::Delete(a) => match small::get_var_name(a.to_string()) {
                Some(a) => {
                    vars.del_var(&a);
                }
                None => {
                    panic!("Error must be a var Line: {}", pos_)
                }
            },
            Command::MkvS(a) => vars.new_var_string(&a[0], &a[1]),
            Command::MakeFile (a,b) => {
                openfile::writeFile(&small::get_value(a,vars.clone()),&small::get_value(b,vars.clone()));
            }
            Command::MakeFolder (a) =>{
                use std::fs;

                fn make_dir(folder: String) -> std::io::Result<()> {
                    fs::create_dir(folder)?;
                    Ok(())
                }
                make_dir(a).unwrap();
            }
            Command::Misc(_) => {}
            _ => {
                panic!("function not found");
            }
        }
    }
}
