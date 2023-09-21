mod cli;

use std::fs::{self, File};

use clap::Parser;
use cli::CliArgs;

fn main() {
  let cli_args = CliArgs::parse();

  let template = fs::read_to_string(cli_args.template_path)
    .expect("Could not read template file");
  let output_file =
    File::open(cli_args.output_path).expect("Could not open output file");
}
