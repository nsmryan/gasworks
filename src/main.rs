
#[macro_use] extern crate quicli;
use quicli::prelude::*;


#[derive(Debug, StructOpt)]
struct Cli {
    infile : String,

    outfile : String,

    #[structopt(flatten)]
    verbosity : Verbosity,
}

main!(|args: Cli, log_level : verbosity| {
    println!("Hello, {} {}!", args.infile, args.outfile);
});

