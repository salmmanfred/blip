//use crate::fun::fun;
#[allow(unused_imports)]
use crate::lib;
use crate::vars::Var;
//use std::io::{self};
use crate::parser::{Command, Parse};
use crate::small;
use openfile;

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
            Command::MakeFile(a, b) => {
                openfile::writeFile(
                    &small::get_value(a, vars.clone()),
                    &small::get_value(b, vars.clone()),
                );
            }
            Command::MakeFolder(a) => {
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
