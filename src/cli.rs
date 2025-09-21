use std::path::PathBuf;

use clap_derive::Parser;
#[derive(Parser, Debug)]
pub struct CommandLine {
    pub file_path: PathBuf,
}
