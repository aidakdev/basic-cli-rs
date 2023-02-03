pub(crate) mod utils;
pub(crate) mod commands;

use clap::Parser;
use anyhow::Result;
use commands::{handle_command, Commands};

#[derive(Debug, Parser)]
#[clap(
	version = "1.0.0",
	name = "basic-cli",
	author = "wiletki",
	about = "⌛️ A basic CLI made using Rust."
)]
pub struct CLI {
	#[clap(subcommand)]
	pub commands: Commands
}

pub async fn run() -> Result<()> {
	let cli = CLI::parse();

	utils::configure_logger();

	if let Err(error) = handle_command(cli.commands).await {
		log::error!("{}", error);
		std::process::exit(1);
	}

	Ok(())
}
