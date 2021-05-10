use crate::vars::var; 

pub fn get_var_name(string: String) -> Option<String> {
    let a = string.split(":").collect::<Vec<&str>>();
    if a[0] == "#MEM" {
        return Some(a[1].to_string());
    }
    None
}
pub fn get_value(string: String,vars: var ) -> String{
    let mut vars = vars;
    let a = string.split(":").collect::<Vec<&str>>();
    if a[0] == "#MEM" {
        return vars.get_var(a[1].to_string())
    }
    return string
}
