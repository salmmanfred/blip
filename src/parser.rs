use crate::small;

#[derive(Clone)]
pub enum Misc {
    Stop,
}
#[derive(Clone)]
pub enum Command {
    Prints(String),
    If([i64; 2], [String; 3]),

    Fn(String, [i64; 2], Vec<String>),
    Loop(String, [i64; 2], Vec<String>, Vec<String>, i64),
    Run(String, [i64; 2], Vec<String>, Vec<String>),

    Misc(Misc),
    Get(String),

    MkvI([String; 2]),
    MkvS([String; 2]),
    Change([String; 2]),
    Delete(String),

    MakeFile(String, String),
    MakeFolder(String),
}
#[derive(Clone)]
pub struct Parse {
    pub parsed_data: Vec<Command>,
}
impl Parse {
    pub fn new() -> Parse {
        Parse {
            parsed_data: Vec::new(),
        }
    }
    pub fn push(&mut self, ty: Command) {
        self.parsed_data.push(ty);
    }
    pub fn find_fn(&mut self, name: &str) -> Command {
        for x in 0..self.parsed_data.len() {
            match self.parsed_data[x].clone() {
                Command::Fn(a, _b, _c) => {
                    if a == name {
                        return self.parsed_data[x].clone();
                    }
                }
                _ => {}
            }
        }
        panic!("no function")
    }
}

pub fn parser(text: Vec<String>) -> Parse {
    let mut parsed = Parse::new();
    let mut curlin = 0;
    for line in text.iter() {
        let mut line = line.split(" ").collect::<Vec<&str>>();
        match line[0] {
            "PR" => {
                let mut print = line;
                print.remove(0);
                let print = print.join(" ");
                parsed.push(Command::Prints(print));
            }
            "IF" => {
                let mut end = 0;
                for lines in curlin..text.len() {
                    let cur_lin = text[lines].clone();
                    let line = cur_lin.split(" ").collect::<Vec<&str>>();
                    if line[0] == "IF_STOP" {
                        end = lines;
                        break;
                    }
                }
                if end == 0 {
                    panic!("forgot stop for IF, use IF_STOP Line: {}", curlin)
                }
                parsed.push(Command::If(
                    [curlin as i64, end as i64],
                    [
                        line[1].to_string(),
                        line[2].to_string(),
                        line[3].to_string(),
                    ],
                ))
            }
            "FN" => {
                let mut end = 0;
                for lines in curlin..text.len() {
                    let cur_lin = text[lines].clone();
                    let line = cur_lin.split(" ").collect::<Vec<&str>>();
                    if line[0] == "FN_STOP" {
                        end = lines;
                        break;
                    }
                }
                if end == 0 {
                    panic!("forgot stop to FN use FN_STOP Line: {}", curlin)
                }
                let name = line[1].to_string();
                line.remove(0);
                line.remove(0);
                let args = line.iter().map(|x| x.to_string()).collect();

                parsed.push(Command::Fn(name, [curlin as i64, end as i64], args));
                //line = vec!("");
            }
            "MKV:S" => {
                let name = line[1].to_string();
                line.remove(0);
                line.remove(0);
                let args: Vec<String> = line.iter().map(|x| x.to_string()).collect();
                let args = args.join(" ");
                parsed.push(Command::MkvS([name, args]))
            }
            "MKV:I" => parsed.push(Command::MkvI([line[1].to_string(), line[2].to_string()])),
            "GET" => {
                match small::get_var_name(line[1].to_string()) {
                    Some(_a) => {}
                    None => {
                        panic!("Must be a variable, Line: {}", curlin)
                    }
                };
                parsed.push(Command::Get(line[1].to_string()))
            }
            "CHA" => {
                match small::get_var_name(line[1].to_string()) {
                    Some(_a) => {}
                    None => {
                        panic!("Must be a variable, Line: {}", curlin)
                    }
                };
                parsed.push(Command::Change([line[1].to_string(), line[2].to_string()]))
            }
            "RUN" => {
                let pos = parsed.find_fn(&line[1]);
                let mut _poss: [i64; 2] = [0, 0];
                let mut _args: Vec<String> = Vec::new();
                match pos {
                    Command::Fn(_a, b, c) => {
                        _poss = b;
                        _args = c;
                    }
                    _ => {
                        panic!(
                            "This should never happen, if it has happen make an issue on github code 1"
                        )
                    }
                }
                line.remove(0);
                line.remove(0);
                let line: Vec<String> = line.iter().map(|x| x.to_string()).collect();
                parsed.push(Command::Run(line[1].to_string(), _poss, line, _args))
            }
            "LOOP" => {
                let pos = parsed.find_fn(&line[1]);
                let mut _poss: [i64; 2] = [0, 0];
                let mut _args: Vec<String> = Vec::new();
                match pos {
                    Command::Fn(_a, b, c) => {
                        _poss = b;
                        _args = c;
                    }
                    _ => {
                        panic!(
                            "This should never happen, if it has happen make an issue on github code 2"
                        )
                    }
                }
                let amount = line[2].parse::<i64>().unwrap();
                line.remove(0);
                line.remove(0);
                line.remove(0);

                let line: Vec<String> = line.iter().map(|x| x.to_string()).collect();
                parsed.push(Command::Loop(
                    line[1].to_string(),
                    _poss,
                    line,
                    _args,
                    amount,
                ))
            }

            "MKFO" => {
                if line.len() > 2 {
                    panic!("MKFO only takes one argument");
                }
                parsed.push(Command::MakeFolder(line[1].to_string()))
            }
            "MKFI" => {
                let name = line[1].to_string();
                line.remove(0);
                line.remove(0);

                let line: Vec<String> = line.iter().map(|x| x.to_string()).collect();
                let line = line.join(" ");
                parsed.push(Command::MakeFile(name, line))
            }
            "DEL" => parsed.push(Command::Delete(line[1].to_string())),
            "STOP" | "IF_STOP" | "FN_STOP" => parsed.push(Command::Misc(Misc::Stop)),
            a => {
                if a != "" {
                    panic!("not a function");
                }
            }
        }
        curlin += 1;
    }
    return parsed;
}
