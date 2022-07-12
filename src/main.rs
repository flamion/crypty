use std::path::PathBuf;
use clap::Parser;
use tracing::{debug, error};
use tracing_core::Level;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Toggles whether debug information should be shown
    #[clap(short = 'D', long)]
	#[clap(default_value = "false")]
	debug: bool,
    /// Which version of encryption to use
	#[clap(short, long)]
    version: String,
	/// The file to be encrypted/decrypted
	#[clap(short, long)]
	file: PathBuf,
	/// The keyfile to be used
	#[clap(short, long)]
	key: PathBuf,
	/// Whether to decrypt or encrypt
	#[clap(short, long)]
	#[clap(default_value = "false")]
	decrypt: bool,
}

fn main() {
	let args = Args::parse();

	let mut env_filter = EnvFilter::from_default_env()
		.add_directive(Level::INFO.into());

	if args.debug {
		env_filter = env_filter.add_directive("encoder_v3=debug".parse().unwrap());
	} else {
		env_filter = env_filter.add_directive("encoder_v3=info".parse().unwrap());
	}

	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_line_number(true)
		.with_file(true)
		.init();

	match args.version.to_lowercase().as_str() {
		"v2" => {
			debug!("Choosing version 2");

		},
		version @ _ => { error!("Invalid version: {version}") }
	}


}

