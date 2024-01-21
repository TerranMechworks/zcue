mod cli;
mod wave;

use cli::{ApplyArgs, Commands, ExtractArgs, MainArgs};
use color_eyre::eyre::{eyre, Context, Result};

fn apply(args: ApplyArgs) -> Result<()> {
    tracing::trace!("reading cue file `{}`", args.cue_file);
    let contents = std::fs::read(&args.cue_file)
        .wrap_err_with(|| format!("Error reading cue file `{}`", args.cue_file))?;
    let cue: wave::CuePoints = serde_json::from_slice(&contents)
        .wrap_err_with(|| format!("Error parsing cue file `{}`", args.cue_file))?;

    println!("Read `{}`", args.cue_file);

    tracing::trace!("reading input file `{}`", args.input_file);
    let buf = std::fs::read(&args.input_file)
        .wrap_err_with(|| format!("Error reading input file `{}`", args.input_file))?;
    let mut wav = wave::read(&buf)
        .wrap_err_with(|| format!("Error parsing input file `{}`", args.input_file))?;

    println!("Read `{}`", args.input_file);

    wave::cue_to_wav(&mut wav, cue)?;

    tracing::trace!("writing output file `{}`", args.output_file);
    let contents = wave::write(&wav).wrap_err("Error constructing output file")?;
    std::fs::write(&args.output_file, contents)
        .wrap_err_with(|| format!("Error writing output file `{}`", args.output_file))?;

    println!("Wrote `{}`", args.output_file);
    Ok(())
}

fn extract(args: ExtractArgs) -> Result<()> {
    tracing::trace!("reading input file `{}`", args.input_file);
    let buf = std::fs::read(&args.input_file)
        .wrap_err_with(|| format!("Error reading input file `{}`", args.input_file))?;
    let wav = wave::read(&buf)
        .wrap_err_with(|| format!("Error parsing input file `{}`", args.input_file))?;

    println!("Read `{}`", args.input_file);

    let mut cue = wave::cue_from_wav(&wav)?;
    if !args.samples {
        cue.sample_points = None;
    }

    tracing::trace!("writing cue file `{}`", args.cue_file);
    let mut contents = serde_json::to_vec_pretty(&cue).wrap_err("Internal error")?;
    contents.push(b'\n');
    std::fs::write(&args.cue_file, contents)
        .wrap_err_with(|| format!("Error writing cue file `{}`", args.cue_file))?;

    println!("Wrote `{}`", args.cue_file);
    Ok(())
}

fn setup_logging() -> Result<()> {
    let env = tracing_subscriber::EnvFilter::from_env("RUST_LOG");

    tracing_subscriber::fmt()
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_writer(std::io::stderr)
        .with_env_filter(env)
        .try_init()
        .map_err(|e| eyre!(e))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    setup_logging()?;

    use clap::Parser as _;
    let args = MainArgs::parse();
    match args.command {
        Commands::Apply(args) => apply(args),
        Commands::Extract(args) => extract(args),
    }
}
