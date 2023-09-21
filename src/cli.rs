use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author = "yarml",
    version = "0.1.0",
    about = "Generate ISO file from a template"
)]
pub struct CliArgs {
    #[arg(name = "template-path")]
    pub template_path: PathBuf,

    #[arg(name = "output-path", short = 'o', long = "output")]
    pub output_path: PathBuf,
}
