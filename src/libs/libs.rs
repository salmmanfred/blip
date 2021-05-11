#[allow(dead_code)]

const STD_LIBS: [&'static str; 1] = [include_str!("std/std.blip")];
#[allow(dead_code)]

const STD_LIBS_NAME: [&'static str; 1] = ["std"];
#[allow(dead_code)]

pub fn find_(name: &str) -> Vec<&str> {
    for x in 0..STD_LIBS.len() {
        if STD_LIBS_NAME[x] == name {
            return STD_LIBS[0].split("\n").collect::<Vec<&str>>();
        }
    }
    panic!("not a library")
}
