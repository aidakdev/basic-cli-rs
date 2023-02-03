use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	basic_cli::run().await?;

	Ok(())
}
