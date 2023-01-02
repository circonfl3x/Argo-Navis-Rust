use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
pub struct MMU{
    pub filepath: String
}

impl MMU{

    pub fn new(fpath: &String) -> MMU {
        MMU{
            filepath: fpath.clone(),
        }
    }

    pub fn gen_var_cache(&self) -> Vec<(String,String,String,String)>{

        use crate::file_management::Fs;
        let mut ret: Vec<(String, String, String, String)> = Vec::new();
        let lines:Vec<String> = Fs::open_file(&Fs{filepath: self.filepath.to_string()});
        for l in lines{

            use crate::useful_stuff::Uo;
            let ln: Vec<String> = Uo::csv(l);
            let op:String;
            let var: String;
            let val: String;
            let typ: String;
            op = ln.get(0).unwrap().to_string();
            var = ln.get(1).unwrap().to_string();
            val = ln.get(2).unwrap().to_string();
            typ = ln.get(3).unwrap().to_string();

            ret.push((op, var, val, typ));

        }

        return ret;
    }

}