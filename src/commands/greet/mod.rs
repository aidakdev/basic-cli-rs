use anyhow::Result;
use clap::Parser;
use log;

#[derive(Debug, Parser)]
#[clap(about = "Says hello to the specified name.")]
pub struct Options {
	#[clap(help = "Name of the person that will be greeted.")]
	name: Option<String>
}

pub async fn handle(options: &Options) -> Result<()> {
	if let Some(ref name) = options.name {
		println!("Hello, {}", name);
	} else {
		log::error!("A name has not been specified.");
		std::process::exit(1);
	}

	Ok(())
}
