extern crate structopt;

use structopt::StructOpt;

mod parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Debug mode
    #[structopt(long = "parse-only")]
    parse_only: bool,
}

fn parse() {
    println!(">>> Parsing...");
    let tree = parser::parse().expect("Error parsing file");

    println!("Resulting tree: {:?}", tree);

    println!("Done!");
}

fn compile() {
    println!(">>> Compiling...");
    // TODO!
    println!("Done!");
}

fn execute() {
    println!(">>> Executing...");
    // TODO!
    println!("Done!");
}

fn main() {
    let opt = Opt::from_args();

    if opt.parse_only {
        parse();
    } else {
        parse();
        compile();
        execute();
    }
}
