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
	#[clap(parse(from_flag))]
	debug: bool,
	/// Which version of encryption to use.
	/// Available versions: 'v2'
	#[clap(short, long, help = clap_version_help())]
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
	/// Toggles whether to decrypt or encrypt.
	/// Encrypts files by default
	#[clap(short, long)]
	#[clap(parse(from_flag))]
	decrypt: bool,
	/// If the specified key file or out file already exists, the program will abort to prevent data loss.
	/// If this flag is set, it skips the check, overwriting the keyfile/outfile, if either or both exist.
	#[clap(short, long, help = clap_ignore_file_check_help())]
	#[clap(parse(from_flag))]
	ignore_existing_files: bool,
}

fn main() {
	let mut env_filter = EnvFilter::from_default_env()
		.add_directive(Level::INFO.into());

	if ARGS.debug {
		env_filter = env_filter.add_directive("crypty=debug".parse().unwrap());
	} else {
		env_filter = env_filter.add_directive("crypty=info".parse().unwrap());
	}

	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_line_number(true)
		.with_file(true)
		.init();

	debug!("Args received: {:#?}", *ARGS);

	debug!("Opening source file: `{}`", &ARGS.source_file.display());
	let mut source_file = std::fs::OpenOptions::new()
		.read(true)
		.write(false)
		.create(false)
		.open(&ARGS.source_file)
		.expect("Couldn't open source file.");

	if !ARGS.ignore_existing_files && !ARGS.decrypt && ARGS.key_file.exists() {
		error!("Key file exists, would override! Aborting!");
		std::process::exit(67);
	}

	if !ARGS.ignore_existing_files && !ARGS.decrypt && ARGS.out_file.exists() {
		error!("Out file exists, would override! Aborting!");
		std::process::exit(67)
	}

	debug!("Opening key file: `{}`", &ARGS.key_file.display());
	let mut key_file = std::fs::OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open(&ARGS.key_file)
		.expect("Couldn't open key file.");
	debug!("Opening out file: `{}`", &ARGS.out_file.display());
	let mut out_file = std::fs::OpenOptions::new()
		.read(false)
		.write(true)
		.create(true)
		.open(&ARGS.out_file)
		.expect("Couldn't open out file.");

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
	debug!("All done");
}


const fn clap_version_help() -> &'static str {
	"Which version of encryption to use"
}

const fn clap_ignore_file_check_help() -> &'static str {
	"If set, skip check for existing key/out file"
}
