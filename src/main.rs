extern crate sha1;
extern crate rustc_serialize;

use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs;

use sha1::Sha1;

struct Blob {
    data:String,
    len : usize,
}

struct Tree {
    Head: Option<Box<Blob>>,
    Tail: (Option<Box<Tree>> , Option<Box<Blob>> , Option<Box<Tree>>),
}


#[derive(RustcEncodable, RustcDecodable)]
struct Repo {
	status:u32,
	files: Option<Vec<String>>,
	dirs: Option<Vec<String>>,
}

impl Repo {
    fn new() -> Self {
        Repo {status:0,files:None,dirs:None}
    }

    // the graphin init cmd
    fn init(&mut self) {
    self.status +=1;
    fn create_dir(name:&str) {
        let metadata = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(name);    
        }
    // create graphin metadata
    fs::create_dir("./.graphin").unwrap();
    create_dir(".graphin/index");
    create_dir(".graphin/object");
    println!("Initialized an empty graphin repository");
    }

    // the graphin status cmd
    // status command as of now checks for existing metadata to evaluate
    fn status(&mut self) {
    match fs::metadata(".graphin") {
        Ok(metadata) => {
            if !(metadata.is_dir()) {
                println!("Not a valid graphin repository");
            } else {
                // code to add file to tracker index
                // if changes in blob object, notify else    
                println!("Working directory clean", );
    }    
        },
        Err(_) => {println!("Not a valid graphin repository");}, 
    }
    // show status of directory here
    }

    // the graphin add cmd
    // TODO
    fn add(&mut self) {

        // reads a file and returns as String
        fn file_buffer(fd:&str) {

        }
    match fs::metadata("./.graphin") {
        Ok(metadata) => {
            if !(metadata.is_dir()) {
                println!("Not a valid graphin repository");
                
            } else {
                // code to add file to tracker index
                // if changes in blob object, notify else    
                let file = std::env::args().nth(2).unwrap();
                // if file == '.' , add all files using walk dir
                // Sha1 instance for hashing current file
                let mut hasher = Sha1::new();
                hasher.update(file.as_bytes());
                println!("{:?} added to master branch SHA1 Hash : {:?}", file, hasher.hexdigest());
                println!("Working directory clean", );

    }    
        },
        Err(_) => {println!("Not a valid graphin repository");}, 
    }  
}

fn branch(&mut self) {
    // dummy repr for now
    println!("Branch:\nOn [Master]\nLatest Commit: a24fcd2d");
    // code to add for branches
}

}

fn main() {
    // replace the repo initialization with rustc-serialize in future
	let mut master:Repo = Repo::new();

	match &std::env::args().nth(1).unwrap()[..] {
	"init" => master.init(),
	"status" => master.status(),
	"add" => master.add(),
	"branch" => master.branch(),
	_ => println!("Command Not Found"),
    }   
}
