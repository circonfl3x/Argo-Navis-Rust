pub struct Uo;


impl Uo{
    pub fn csv(ln: String) -> Vec<String>{
        ln.split(",")
        .map(|x| x.trim().to_string())
        .collect()

    }
}