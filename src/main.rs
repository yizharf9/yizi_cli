use std::env;
use std::fs;
use std::process;
use std::error::Error;

use yizi_cli::Config;

fn main() {
    println!("");
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("{err}");
        println!("");
        process::exit(1);
    });
        
    // ("something went wrong with the file content!")
    println!("searching for query : {}",&config.query);
    println!("in -> |{}|\n",&config.filename);

    run(config).unwrap_err();
    // if let Err(e) =  run(config){
    //     println!("{e}")
    // }else {
    //     println!("ran successfully!")
    // }
}

    
fn run(config:Config)->Result<(),Box<dyn Error>> {   
    let content = fs::read_to_string(&config.filename)?;

    println!("{}",&config.filename);
    println!("{}",&content);
    println!("");
    
    Ok(())
}
