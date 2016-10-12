
pub mod input;
pub mod net;
pub mod output;

extern crate hyper;
extern crate getopts;

use getopts::Options;
use std::env;
use input::parse;
use net::Builder;
use output::elasticsearch::Elastic;

const DEFAULT_NUM_TIMES: u32 = 5;
const NUM_PRODUCERS: u8 = 6;
const NUM_CONSUMERS: u8 = 3;

fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options] (<file> || (<url> && <name>))", program)));
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this usage message.");
    opts.optopt("f", "file", "Specifies the file which contains the URLs.", "FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let file_path = matches.opt_str("f");
    let queries = match parse::read_file(&file_path, DEFAULT_NUM_TIMES) {
        Ok(m)  => { m }
        Err(e) => { panic!(e.to_string()) }
    };

    Builder::new()
        .num_consumers(NUM_CONSUMERS)
        .num_producers(NUM_PRODUCERS)
        .build(queries)
        .perform_queries(Elastic::factory());
}
