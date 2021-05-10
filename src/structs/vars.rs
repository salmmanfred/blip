
#[derive(Clone)]
pub struct var {
    pub string: Vec<String>,
    pub string_name: Vec<String>,
    // ints and then strings ^
    pub is: Vec<String>,
    pub is_name: Vec<String>,
}
impl var {
    pub fn new() -> var {
        var {
            string: Vec::new(),
            string_name: Vec::new(),
            is: Vec::new(),
            is_name: Vec::new(),
        }
    }
    //make a new var thats a string the only 2 variables in this language
    pub fn new_var_string(&mut self, name: &str, value: &str) {
        self.string.push(value.to_string());
        self.string_name.push(name.to_string());
    }
    // get a var from memory #MEM:var
    pub fn get_var(&mut self, name: String) -> String {
        for x in 0..self.string_name.len() {
            if self.string_name[x] == name {
                return self.string[x].clone();
            }
        }
        for x in 0..self.is_name.len() {
            if self.is_name[x] == name {
                return self.is[x].to_string().clone();
            }
        }
        panic!("no var");
        "".to_string()
    }
    //make a new variable
    pub fn new_var_i(&mut self, name: &str, value: &str) {
        self.is.push(value.to_string());
        self.is_name.push(name.to_string());
    } //update the variable
    pub fn up_var(&mut self, name: &str, new_val: &str) {
        for x in 0..self.string_name.len() {
            if self.string_name[x] == name {
                self.string[x] = new_val.to_string();
            }
        }
        for x in 0..self.is_name.len() {
            if self.is_name[x] == name {
                self.is[x] = new_val.to_string();
            }
        }
    }
    pub fn del_var(&mut self, name: &str) {
        for x in 0..self.string_name.len() {
            if self.string_name[x] == name {
                self.string.remove(x);
                self.string_name.remove(x);
            }
        }
        for x in 0..self.is_name.len() {
            if self.is_name[x] == name {
                self.is.remove(x);
                self.is_name.remove(x);
            }
        }
    }
}
