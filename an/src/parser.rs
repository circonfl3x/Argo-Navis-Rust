pub struct Parser {
    pub line_number: u32,
    pub op: String,
    pub val: String,
    pub var: String,
    pub fpath: String,
}

impl Parser {
    pub fn new(ln: &u32, op: &String, val: &String, var: &String, fpath: &String) -> Parser {
        Parser {
            line_number: *ln,
            op: op.clone(),
            val: val.clone(),
            var: var.clone(),
            fpath: fpath.clone(),
        }
    }

    pub(self) fn get_type(&self) -> String {
        match self.val.parse::<u32>() {
            Ok(t) => return "Int".to_string(),
            Err(_) => (),
        }

        match self.val.chars().filter(|x| *x == '"').count() == 2 {
            true => return "String".to_string(),
            false => (),
        }

        return "Uknown".to_string();
    }
    pub(self) fn mov(&self) -> () {
        // i'll be looking to move these to file_management later
        use std::fs::{File, OpenOptions};
        use std::io::Write;
        let mut f: File;
        match File::open(".argo_cache") {
            Ok(_) => f = OpenOptions::new().append(true).open(".argo_cache").unwrap(),
            Err(_) => panic!("Couldn't open argo_cache"),
        }

        match self.var.parse::<u32>() {
            Ok(_) => panic!(
                "Variable name cannot be an integer! line: {}",
                self.line_number
            ),
            Err(_) => (),
        };

        //check if string is using right string assignment
        if self.get_type() == "String" {
            if self.val.chars().filter(|x| *x == '"').count() == 2 {
            } else {
                panic!("Invalid string configuration on line: {}", self.line_number);
            }
        }

        f.write(
            format!(
                "{},{},{},{}\n",
                self.op,
                self.var,
                self.val,
                self.get_type()
            )
            .as_bytes(),
        )
        .unwrap();

        f.sync_all().expect("Couldn't sync file ops with disk!!! parser.rs");

        if self.var == "stdout" {
            self.stdout();
        }
    }

    pub(self) fn is_valid_string(&self) -> bool {
        if self.get_type() == "String" {
            if self.val.chars().filter(|x| *x == '"').count() == 2 {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    pub(self) fn call(&self) {
        println!("This does nothing as of now. !!Deprecated function!!");
        if self.op == "stdout" {}
    }

    pub(self) fn stdout(&self) {
        use crate::base_lib::stdout;

        if self.get_type() == "String" && self.is_valid_string() {
            stdout::Stdout::stdout::<String>(&self.val);
        } else{
            use crate::memory_management::MMU;
            let vect: Vec<_> = MMU::gen_var_cache(&MMU::new(&".argo_cache".to_string()));

            for ln in vect {
                if ln.3 == "Int"{
                    stdout::Stdout::stdout::<String>(&self.val);
                }
                if self.var == ln.1 {
                    // we can't just call for the value in ln.2 because that is the name of the variable that is stored so we have to loop over the
                    // entire code file so we can check... How fun
                    use crate::file_management;
                    use crate::tokenizer;
                    let arr_lines:Vec<_> = file_management::Fs::open_file(&file_management::Fs::new(&self.fpath));
                    for r in arr_lines{
                        let var:String;
                        let val:String;

                        //we only need the variable name and the value, the operator is uneeded hence is uninitialized
                        (_,var,val) = tokenizer::Tokenizer::experimental_seperate(&tokenizer::Tokenizer::new(&self.line_number, &r));
                        if var == ln.1{
                            stdout::Stdout::stdout::<String>(&val);
                        }
                    }
                }
            }
        }
    }

    pub fn function(&self) {
        match self.op.as_ref() {
            "mov" => self.mov(),
            "call" => self.call(),
            "stdout" => self.stdout(),

            _ => (),
        }
    }
}
