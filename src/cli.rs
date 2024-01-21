use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Manipulate cue points in wave files",
    long_about = None,
    arg_required_else_help(true),
)]
pub(crate) struct MainArgs {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Args)]
pub(crate) struct ExtractArgs {
    /// The wave file to read from
    pub(crate) input_file: Utf8PathBuf,
    /// The cue file to write cue points to
    pub(crate) cue_file: Utf8PathBuf,
    /// Specify to write sample information
    #[clap(long, action)]
    pub(crate) samples: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ApplyArgs {
    /// The wave file to read from
    pub(crate) input_file: Utf8PathBuf,
    /// The cue file to read cue points from
    pub(crate) cue_file: Utf8PathBuf,
    /// The wave file to write to
    pub(crate) output_file: Utf8PathBuf,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Extract cue points from a wave file
    #[command(arg_required_else_help(true))]
    Extract(ExtractArgs),
    /// Apply cue points to a wave file
    #[command(arg_required_else_help(true))]
    Apply(ApplyArgs),
}
