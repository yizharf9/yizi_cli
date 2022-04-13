
pub struct Config{
    pub query : String,
    pub filename : String,
    pub readable : bool
}

impl Config {
    pub fn new(args : &[String])->Result<Config,&str> {
        if args.len()<3 {
            return Err("phrase not matching required length!")
        }
        
        // for line in args.iter() {
        //     println!("{:?}",line);
        //     println!("");
        // }

        let query= args[1].clone();
        let filename= args[0].clone();
        let readable = args[2] == "read";

        Ok(Config{filename,query,readable})
    }
}

#[allow(dead_code)]
// fn search(q:&str,c:&str)->bool {
fn search<'a>(q:&str,c:&'a str)->&'a str {
    let mut counter = 0;
    for line in c.split("\n") {

        if line.contains(q) {
            println!("phrase found in line : {counter}");
            return line;
            // return true;
        };

        println!("searching in line : {counter}");
        counter+=1;
    }
    "phrase not found :("
    // false
}

#[cfg(test)]
mod tests{
    use std::fs;
    use super::*;    

    #[test]
    fn test_1() {
        let query = "you";
        let content = fs::read_to_string("data.txt").unwrap_or(String::from("incorrect input"));
        println!("{content}");
        println!("");

        // assert_eq!(search(query, &content),true)
    }
    #[test]
    fn test_2() {
        let query = "yizhar";
        let content = fs::read_to_string("data.txt").unwrap_or(String::from("incorrect input"));
        println!("{content}");
        println!("");

        // assert_eq!(search(query, &content),true)
    }
}


#[allow(dead_code)]
fn main() {
    // let resault =
    let resault = search("q", "g\nj\njy\ni\no\nr\nq");
    println!("{resault}");
}