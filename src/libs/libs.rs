const std_libs: [&'static str; 1] = [include_str!("std/std.blip")];
const std_libs_name: [&'static str; 1] = ["std"];
pub fn find_(name: &str) -> Vec<&str> {
    for x in 0..std_libs.len() {
        if std_libs_name[x] == name {
            return std_libs[0].split("\n").collect::<Vec<&str>>();
        }
    }
    panic!("not a library")
}
