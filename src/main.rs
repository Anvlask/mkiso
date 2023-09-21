mod cli;
mod template;

use std::fs::{self, File};

use clap::Parser;
use cli::CliArgs;
use template::PartitionTemplate;
use toml::Table;

fn main() {
  let cli_args = CliArgs::parse();

  let template_raw = fs::read_to_string(cli_args.template_path)
    .expect("Could not read template file");
  let _output_file =
    File::create(cli_args.output_path).expect("Could not open output file");

  let template = template_raw
    .parse::<Table>()
    .expect("Could not parse template file");

  let partitions = template.iter().map(|(partname, val)| {
    PartitionTemplate::from_table(
      partname,
      val
        .as_table()
        .expect(&format!("Invalid partition {}", partname)),
    )
  });

  partitions.for_each(|partition| {
    dbg!(partition);
  });
}
