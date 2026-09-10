use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use color_eyre::Result;
use rust_template_project::Greeter;
use std::io;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// A modern Rust CLI template demonstrating best practices.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet.
    #[arg(short, long, default_value = "World")]
    name: String,

    /// Use a formal greeting instead of a casual one.
    #[arg(short, long)]
    formal: bool,

    /// Increase logging verbosity (-v for debug, -vv for trace).
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Generate shell completion script to stdout for the specified shell.
    #[arg(long, value_name = "SHELL")]
    generate_completions: Option<Shell>,
}

fn init_tracing(verbosity: u8) {
    let default_filter = match verbosity {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn print_completions(shell: Shell, cmd: &mut clap::Command) {
    let bin_name = cmd.get_name().to_string();
    generate(shell, cmd, bin_name, &mut io::stdout());
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if let Some(shell) = cli.generate_completions {
        let mut cmd = Cli::command();
        print_completions(shell, &mut cmd);
        return Ok(());
    }

    init_tracing(cli.verbose);

    tracing::debug!("Starting greeting CLI application");

    let greeter = Greeter::new(&cli.name)?;

    if cli.formal {
        println!("{}", greeter.greet_formal());
    } else {
        println!("{}", greeter.greet());
    }

    tracing::debug!("Finished greeting successfully");
    Ok(())
}
