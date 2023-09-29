use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input path of project
    #[arg(long, default_value_t = format!("."))]
    input: String,

    /// Which binary file to use. Required if there are several targets in Cargo.toml
    #[arg(short, long)]
    binary: Option<String>,
}

fn main() {
    let args = Args::parse();

    let path = PathBuf::from(&args.input);

    match cargo_bundler::bundle_project(&path, args.binary) {
        Ok(code) => println!("{code}"),
        Err(reason) => eprintln!("ERROR: {reason}"),
    }
}
