use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target directory to export Signal message attachments
    #[arg(short, long)]
    pub target_directory: String,
}
