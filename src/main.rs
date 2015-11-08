use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;

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
	println!("Initialized a graphin repository");
// create directory here;
}

fn status(repo_instance:&Repo) {
	if (repo_instance.status == 0) {
		println!("Fatal Error: Not a graphin repository");
	} else { 
	println!("Working directory clean");
	}
// show status of directory here
}

fn add() {
	println!("Added to graphin repository");
// code to add file to tracker index
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
	"add *.*" => add(),
	"branch" => branch(),
	_ => println!("Command Not Found"),
}   
}
