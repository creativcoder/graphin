use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs;

struct Repo {
	status:u32,
	files: Option<Vec<String>>,
	dirs: Option<Vec<String>>,
}

impl Repo {
	fn new() -> Self {
		Repo {status:0,files:None,dirs:None}
	}
}

fn init(repo_instance:&mut Repo) {
	repo_instance.status +=1;
    let metadata = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(".graphin");
    println!("Initialized an empty graphin repository");
}

fn status(repo_instance:&Repo) {

	match fs::metadata(".graphin") {
        Ok(metadata) => {
            if !(metadata.is_file()) {
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

fn add(repo_instance:&mut Repo) {

    match fs::metadata(".graphin") {
        Ok(metadata) => {
            if !(metadata.is_file()) {
                println!("Not a valid graphin repository");
                
            } else {
                // code to add file to tracker index
                // if changes in blob object, notify else    
                let file = std::env::args().nth(2).unwrap();
                println!("{:?} added to master branch", file);
                println!("Working directory clean", );

    }    
        },
        Err(_) => {println!("Not a valid graphin repository");}, 
    }

    
}

fn branch() {
	println!("Branches:");
// code to add for branches
}

fn main() {
	let mut master:Repo = Repo::new();
	match &std::env::args().nth(1).unwrap()[..] {
	"init" => {init(&mut master);},
	"status" => status(&master),
	"add" => add(&mut master),
	"branch" => branch(),
	_ => println!("Command Not Found"),
}   
}
