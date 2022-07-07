// use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use chrono::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "garden", about = "Building your MDX template")]
enum Garden {
    New {
        /// Activate debug mode
        // short and long flags (-d, --debug) will be deduced from the field"s name
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

#[derive(PartialEq, Debug)]
enum MessageType {
    Open,
    ReadJSON,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            MessageType::Open => "Failed to open file",
            MessageType::ReadJSON => "Failed to parse json",
        };
        f.write_str(description)
    }
}

#[derive(Debug)]
struct ConfigError {
    err_type: MessageType,
    err_message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = format!("{}", &self.err_type);
        f.write_str(&format!("{}: {}", description, self.err_message).to_string())
    }
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).map_err(|e| ConfigError {
        err_type: MessageType::Open,
        err_message: format!("{}", e),
    })?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let config = serde_json::from_reader(reader).map_err(|e| ConfigError {
        err_type: MessageType::ReadJSON,
        err_message: format!("{}", e),
    })?;

    // Return the `Config`.
    Ok(config)
}

#[derive(Serialize)]
struct Frontmatter {
    draft: bool,
    title: String,
    path: String,
    date: String,
    author: String,
    description: String,
    categories: Vec<String>,
    keywords: Vec<String>,
    garden: String,
    image: String,
}

fn main() {
    match Garden::from_args() {
        Garden::New { debug, input_path } => {
            if debug {
                println!("args: {}, {:?}", debug, input_path);
            };
            let config = match read_config_from_file(input_path) {
                Ok(config) => config,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };
            println!("{:#?}", config);
        }
    };
    let frontmatter = Frontmatter {
        draft: true,
        title: "This is a String".to_string(),
        path: "/garden/{{slug}}".to_string(),
        date: Utc::now().to_string(),
        author: "talves".to_string(),
        description: "This is just the description".to_string(), // multi-line  {{description}}
        categories: ["draft".to_string()].to_vec(),              // - "draft"
        keywords: ["new".to_string(), "garden".to_string()].to_vec(),
        garden: "sprout".to_string(),
        image: "/images/social/{{slug}}.png".to_string(),
    };
    let toml = toml::to_string(&frontmatter).unwrap();
    println!("---\n{}---\n\n## First sub Title", toml);
}
