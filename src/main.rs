use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "garden", about = "Building your MDX template")]
enum Garden {
    New {
        /// Activate debug mode
        // short and long flags (-d, --debug) will be deduced from the field's name
        #[structopt(short, long)]
        debug: bool,

        /// Input file
        #[structopt(short, parse(from_os_str))]
        input_path: PathBuf,
    },
}

#[derive(Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    title: String,
    authors: Vec<Author>,
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let config = serde_json::from_reader(reader)?;

    // Return the `Config`.
    Ok(config)
}

fn main() {
    match Garden::from_args() {
        Garden::New { debug, input_path } => {
            println!("{}, {:?}", debug, input_path.to_str(),);
            let config = read_config_from_file(input_path).unwrap();
            println!("{:#?}", config);
        }
    };
}
