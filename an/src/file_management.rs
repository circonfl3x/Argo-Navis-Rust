use std::fs::File;
use std::io::*;
use std::io::Result;
use std::path::{Path,PathBuf};


pub struct Fs{
    pub filepath: String
}


impl Fs
{
    pub fn new(fpath: &String) -> Fs{
        Fs{
            filepath: fpath.clone()
        }
    }
    pub (self) fn read (&self) -> Result<Lines<BufReader<File>>>

    {
        let file = File::open(&self.filepath)?;
        Ok(BufReader::new(file).lines())
    }

    pub fn open_file (&self) -> Vec<String>
    {
        let mut out:Vec<String> = Vec::new();
        let buff: Result<Lines<BufReader<File>>> = self.read();
        if let Ok(fpath) = buff{
                for lines in fpath{
                    match lines{
                        Ok(t) => out.push(t),
                        Err(_)=>panic!("cannot read open_file(): filemanagement_rs/30")
                    };


                }
            }
            out
        }

    }

