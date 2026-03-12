use std::env;
use std::error::Error;
use std::fs;
use std::process;
use scanner_rust::ScannerStr;

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
   
    if let Err(e) = run(config) {
        println!("Error : {e}");
        process::exit(1)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut contents = fs::read_to_string(config.file_path)?;
    let mut scanner = ScannerStr::new(&contents);

    for token in scanner {
       println!("{token}"); 
    }

    Ok(())
}

struct Config {
    pub file_path: String,
}

impl Config {
    fn build (mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
            let file_path: String = match args.next() {
                Some(arg) => arg,
                None => return Err("Did not get a file path"), 
            };

        Ok(Config{file_path:file_path})
    }
}
