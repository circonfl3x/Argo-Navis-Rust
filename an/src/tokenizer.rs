pub struct Tokenizer {
    pub line_number: u32,
    pub line: String,
}

impl Tokenizer {
    pub fn new(line_number: &u32, line: &String) -> Tokenizer {
        Tokenizer {
            line_number: *line_number,
            line: line.clone(),
        }
    }
    pub(self) fn elems<T>(&self, arr: &Vec<T>) -> u8 {
        let mut count = 0;

        for _ in arr {
            count += 1;
        }

        count
    }
    pub fn separate(&self) -> (String, String, String) {
        let mut operator: String = String::new();
        let mut var: String = String::new();
        let mut value: String = String::new();

        let splitcomma: Vec<String> = self.line.split(",").map(|x| x.to_string()).collect();

        if self.elems(&splitcomma) < 2 || self.elems(&splitcomma) > 3 {
            panic!("Error: wrong syntax on line {}", self.line_number);
        }
        if self.elems(&splitcomma) == 2 {
            println!("This will only evaluate for call");
        }
        if self.elems(&splitcomma) == 3 {
            operator = splitcomma.get(0).unwrap().trim().to_string();

            var = splitcomma.get(1).unwrap().trim().to_string();

            value = splitcomma.get(2).unwrap().trim().to_string()
        }

        (operator, var, value)
    }

    pub fn experimental_seperate(&self) -> (String, String, String) {
        let operator: String;
        let var: String;
        let value: String;

        {
            let split_at_whitespace: Vec<String>;
            split_at_whitespace = self.line.split(" ").map(|x| x.to_string()).collect();
            if split_at_whitespace.len() < 2{
                panic!("Incorrect use of whitespaces on line {}", self.line_number);
            }
            operator = split_at_whitespace.get(0).unwrap().trim().to_string();
        }

        {
            let split_at_comma: Vec<String>;
            split_at_comma = self.line.split(",").map(|x| x.to_string()).collect();
            if split_at_comma.iter().count() < 2 {
                println!("{:#?}", split_at_comma);
                panic!("Incorrect use of commas at line {}", self.line_number);
            }
            var = split_at_comma
                .get(0)
                .unwrap()
                .replace(&format!("{}", operator), "")
                .trim()
                .to_string();

            value = split_at_comma.get(1).unwrap().trim().to_string();
        }

        (operator, var, value)
    }
}
