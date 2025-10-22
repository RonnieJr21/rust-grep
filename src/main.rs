use std::{io::{BufRead, BufReader}, iter::Map};


fn main() {

    let args:Vec<String> = std::env::args().collect();
    let cli = Cli::new(args);
    println!("{:?}", cli.deep_search());

}
struct Cli {
    pattern: String,
    path: std::path::PathBuf,

}



impl Cli {
    fn new(args: Vec<String>) -> Cli {
        
        let mut iter = args.into_iter().skip(1);
        let pattern = iter.next().expect("No pattern provided");
        let path = iter.next().expect("No path provided").into();

        Cli {
            pattern,
            path,
        }
    }
       
    fn quick_search(&self){

    let file = std::fs::File::open(&self.path).expect("Could not open file");
    let file_reader = BufReader::new(file);

        for line in file_reader.lines() {
            let line = line.expect("Could not read line");
            if line.contains(&self.pattern) {
                println!("{}", line);
            }

 
        }
    }

    fn deep_search(&self) /* -> Vec<String> */ {
        let root = std::path::Path::new("/");
        let mut queue = Queue::new();

        queue.enqueue(root.to_path_buf());

        for dir in queue.dequeue() {
           if dir.is_dir() {
                for entry in std::fs::read_dir(dir).expect("Could not read directory") {
                     let entry = entry.expect("Could not get directory entry");
                     let path = entry.path();
                     if path.is_dir() {
                        println!("directory: {:?}", &path);
                        queue.enqueue(path);
                          
                     } else {
                            let file = std::fs::File::open(&path).expect("Could not open file");
                            let file_reader = BufReader::new(file);

                                for line in file_reader.lines() {
                                    let line = line.expect("Could not read line");
                                    if line.contains(&self.pattern) {
                                        println!("Found in {:?}: {}", &path, line);
                                    }
                                }
                          }
                     }
                }
           }
        }

        

    }


struct Queue <T> {
    items: Vec<T>
}

impl <T> Queue <T> {
    fn new() -> Queue<T> {
        Queue {
            items: Vec::new()
        }
    }
    fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }
    fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
      fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
   
}
