use crate::openfile;
mod libs;

pub fn load(text: Vec<String>) -> Vec<String> {
    let mut text = text;
    for OP in 0..text.len() {
        let split_OP = text[OP].split(" ").collect::<Vec<&str>>();
        if split_OP[0] == "INC" {
            if split_OP[1] == "STD" {
                let lib = libs::find_(split_OP[2]);
                let mut lib: Vec<String> = lib.iter().map(|x| x.to_string()).collect();
                lib.retain(|x| x != "");
                text.extend(lib);
            } else {
                let mut lib_load = openfile::readFileLines(split_OP[1]);
                lib_load.retain(|x| x != "");
                text.extend(lib_load);
            }
        }
    }
    return text;
}
