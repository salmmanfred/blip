use crate::openfile;
mod libs;
#[allow(dead_code)]

pub fn load(text: Vec<String>) -> Vec<String> {
    let mut text = text;
    for o_p in 0..text.len() {
        let split_op = text[o_p].split(" ").collect::<Vec<&str>>();
        if split_op[0] == "INC" {
            if split_op[1] == "STD" {
                let lib = libs::find_(split_op[2]);
                let mut lib: Vec<String> = lib.iter().map(|x| x.to_string()).collect();
                lib.retain(|x| x != "");
                text.extend(lib);
            } else {
                let mut lib_load = openfile::readFileLines(split_op[1]);
                lib_load.retain(|x| x != "");
                text.extend(lib_load);
            }
        }
    }
    return text;
}
