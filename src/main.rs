use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use lazy_static::lazy_static;
use tracing::{debug, error};
use tracing_core::Level;
use tracing_subscriber::EnvFilter;

mod v2;

lazy_static! {
	static ref ARGS: Args = Args::parse();
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
	/// Toggles whether debug information should be shown.
	#[clap(short = 'D', long)]
	#[clap(default_value = "false")]
	debug: bool,
	/// Which version of encryption to use.
	#[clap(short, long, long_help = clap_version_long_help())]
	version: String,
	/// The file to be encrypted/decrypted.
	#[clap(short = 'f', long)]
	source_file: PathBuf,
	/// The keyfile to be used.
	#[clap(short, long)]
	key_file: PathBuf,
	/// Encrypted/Decrypted output file.
	#[clap(short = 'o', long)]
	out_file: PathBuf,
	/// Whether to decrypt or encrypt.
	#[clap(short, long)]
	#[clap(default_value = "false")]
	decrypt: bool,
}

fn main() {
	let mut env_filter = EnvFilter::from_default_env()
		.add_directive(Level::INFO.into());

	if ARGS.debug {
		env_filter = env_filter.add_directive("encoder_v3=debug".parse().unwrap());
	} else {
		env_filter = env_filter.add_directive("encoder_v3=info".parse().unwrap());
	}

	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_line_number(true)
		.with_file(true)
		.init();

	let mut source_file = File::open(&ARGS.source_file).expect("Couldn't open source file.");
	let mut key_file = File::open(&ARGS.key_file).expect("Couldn't open key file.");
	let mut out_file = File::open(&ARGS.out_file).expect("Couldn't open out file.");

	if let Err(why) = match ARGS.version.to_lowercase().as_str() {
		"v2" => {
			debug!("Choosing version 2");
			v2::process_file(&mut source_file, &mut key_file, &mut out_file)
		}
		version => {
			error!("Invalid version: {version}");
			std::process::exit(68);
		}
	} {
		error!("An error occurred while processing: {why}");
	}
}


const fn clap_version_long_help() -> &'static str {
	"Which version of encryption to use\nAvailable versions: 'v2'"
}
