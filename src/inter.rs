
pub fn run(text: Vec<String>){
    let mut vars = var::new();
    let mut funs = fun::new();
    funs.find_fun(text.clone());
    // test vars
    // all variables are public 
    vars.new_var_string("LK","t");
    vars.new_var_string("RK","t2");
    //run the code
    parse_run([0,text.clone().len() as i64],text,&mut vars,&mut funs);

   
}
struct var{
    string: Vec<String>,
    string_name: Vec<String>,
    // ints and then strings ^
    is: Vec<String>,
    is_name: Vec<String>,
}
impl var{
    pub fn new()-> var{
        var{
            string: Vec::new(),
            string_name: Vec::new(),
            is: Vec::new(),
            is_name: Vec::new(),
        }
    }
    //make a new var thats a string the only 2 variables in this language 
    pub fn new_var_string(&mut self,name: &str, value: &str){
        self.string.push(value.to_string());
        self.string_name.push(name.to_string());
    }
    // get a var from memory #MEM:var
    pub fn get_var(&mut self,name:String)->String{

        for x in 0..self.string_name.len(){
            if self.string_name[x] == name{
                return self.string[x].clone()
            }
        }
        for x in 0..self.is_name.len(){
            if self.is_name[x] == name{
                return self.is[x].to_string().clone()
            }
        }
        panic!("no var");
        "".to_string()

    }
    //make a new variable 
    pub fn new_var_i(&mut self,name: &str, value: &str){
        self.is.push(value.to_string());
        self.is_name.push(name.to_string());
    }//update the variable 
    pub fn up_var(&mut self, name: &str, new_val: &str){
        for x in 0..self.string_name.len(){
            if self.string_name[x] == name{
                self.string[x] = new_val.to_string();
            }
        }
        for x in 0..self.is_name.len(){
            if self.is_name[x] == name{
                self.is[x] = new_val.to_string();
            }
        }
    }
}

struct fun{
    names: Vec<String>,
    start: Vec<i64>,
    end: Vec<i64>
}
impl fun{
    // find all the functions and put them in the struct 
    pub fn find_fun(&mut self,text: Vec<String>){
        let mut find = true;
        for x in 0..text.len(){
            let split = text[x].split(" ").collect::<Vec<&str>>();
            if find{
                if split[0] == "FN"{
                    self.names.push(split[1].to_string());
                    self.start.push(x as i64);
                    find = false;
                   

                }
            }else{
                if split[0] == "STOP_FN"{
                    self.end.push(x as i64);
                    find = true;
                }
            }

        }
    } 
    // get the last row of the function 
    pub fn get_last(&self,name: &str)->usize{
        for x in 0..self.names.len(){
            if self.names[x] == name{
                return self.end[x] as usize;
            }
        }
        panic!("No function with that name");
    }
    // get the first row of the function 
    pub fn get_first(&self,name: &str)->usize{
        for x in 0..self.names.len(){
            if self.names[x] == name{
                return self.start[x] as usize;
            }
        }
        panic!("No function with that name");
    }
    // run a function 
    pub fn run(&mut self, name:&str,text: Vec<String>, vars: &mut var){
        parse_run([self.get_first(name) as  i64+ 1 as i64,self.get_last(name) as i64], text, vars,self);
    }
    
    pub fn new()->fun{
        fun{
            names: Vec::new(),
            start: Vec::new(),
            end: Vec::new(),
        }
    }
}
fn parse_run(se:[i64;2], text: Vec<String>,vars:&mut var, funs: &mut fun){
    let mut i: bool = false;
    let mut modif: i64 = 0;
    for OP in se[0]..se[1] as i64{
        let OP = (OP + modif) as usize;
        if OP >= text.len(){// so that it dose not over run itself
            break;
        }
        let split_OP = text[OP].split(" ").collect::<Vec<&str>>();
        if split_OP[0] == "IF"{// go into the if i if this is true
            i = true;
            
        }
        // this runs if its an if
        if i{
            let mut comp1S = "".to_string();
            let mut comp2S = "".to_string();
            let mut moda = 0;
            let mut eqfun = "";
            let mut iftrue = false;
             { // get comp1
                let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM"{
                    comp1S = vars.get_var(a[1].to_string().clone());
                }else{
                    comp1S = split_OP[1].to_string().clone();
                }
                
            }
            {//if it should be == or something else
                eqfun = split_OP[2]
            }
            {
               
                let a = split_OP[3].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM"{
                    comp2S = vars.get_var(a[1].to_string().clone());
                }else{
                    comp2S = split_OP[3].to_string().clone();
                }
         
                moda = 3;
                
            }

            // stop if it should stop
            if text[OP] == "STOP"{
                i = false; 
            }
            // do the if 
            if moda == 3{
                
                if eqfun == "=="{
                   if comp1S == comp2S{
                        iftrue = true;
                    }
                }
                if eqfun == "!="{
                    if comp1S != comp2S{
                         iftrue = true;
                     }
                 }

            }
           
            // if it turns out to be true it runs the things in the if 
            if iftrue{
               
                /*iftrue = false;
                eqfun = "";
                moda = 0;
                comp1S = "".to_string();
                comp2S = "".to_string();*/
                i = false;
                //modif -= 1;

            }
            
            
           

        }else{
            // print 
            if split_OP[0] == "PR"{
               // let a = text[OP+1].split(":").collect::<Vec<&str>>();
               let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM"{
                    println!("{}",vars.get_var(a[1].to_string()));

                }else{
                    println!("{}",split_OP[1]);
                }
                //modif += 1; 
            }
            // make a variable 
             if split_OP[0]== "MKV:S"{
                
                vars.new_var_string(&split_OP[1],&split_OP[2]);
               // modif += 2;
             }
             // make a new i variable 
            if split_OP[0] == "MKV:I"{
                
                vars.new_var_i(&split_OP[1],&split_OP[2]);
               // modif += 2;
            }
            // add a number to a number 
            if split_OP[0] == "ADD"{
                let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM"{
                    let add = vars.get_var(a[1].to_string()).parse::<i64>().unwrap();
                    
                    let amount = split_OP[2].parse::<i64>().unwrap();
                    let total = add+amount;
                   
                    vars.up_var(a[1], &total.to_string());
                   // modif += 2;
                }else{
                    panic!("WHAT");
                }
            }
            // change the variable so like var = thisnow 
            if split_OP[0] == "CHA"{
                let a = split_OP[1].split(":").collect::<Vec<&str>>();
                if a[0] == "#MEM"{
                    

                    vars.up_var(a[1], &split_OP[2]);
                    //modif += 2;
                }else{
                    panic!("WHAT");
                }
            }
            // if it detects a fun it jumps over it 
            if split_OP[0] == "FN"{
                let x = funs.get_last(split_OP[1]);
                modif += x as i64-OP as i64;
            }
            // if it finds the run key it runs the function 
            if split_OP[0] == "RUN"{
                funs.run(split_OP[1], text.clone(), vars);
            }
        }
       
    }
}
