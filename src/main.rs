use std::io::Error;
use std::path::PathBuf;
use std::fs;
use std::process::exit;

fn main() {

    let cli = Cli::new(std::env::args().collect());

    let word = Cli::parse(&cli).unwrap();

    println!("Finished! {:?}", word);

}
#[derive(Debug)]
struct Cli {
    pattern: String,
    path: PathBuf,
}


impl Cli {
    fn new(args: Vec<String>) -> Cli {
        if args.len() < 3 {
            Cli::error( 0)
        }
        
        let pattern =  args.iter().nth(1).unwrap().clone();
        let path:PathBuf = PathBuf::from(args.iter().nth(2).unwrap().clone());
        println!("path: {:?}", path);
        Cli { pattern, path }
    }

    fn parse(&self)  -> std::io::Result<String> {
        let content = fs::read_to_string(self.path.as_path())?;
        let mut word: String = String::new();
        for line in content.lines() {
            if(line.contains(self.pattern.as_str())) {
                word = String::from(line);
            }
        }
        Ok(word)
    }

    fn error(code: i8)  {

        match &code {
            0 => {println!("[Not enough arguments]\nUsage : cargo run -- [pattern] [path]");
                            exit(0)},
            _ => println!("")
        }
    }
}
