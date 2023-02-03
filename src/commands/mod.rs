mod greet;

use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
	Greet(greet::Options)
}

pub async fn handle_command(command: Commands) -> Result<()> {
	match command {
		Commands::Greet(options) => greet::handle(&options).await
	}
}
