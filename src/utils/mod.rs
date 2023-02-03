use log::{Level, LevelFilter};
use fern::colors::{Color, ColoredLevelConfig};

pub fn configure_logger() {
	let colors = ColoredLevelConfig::new()
		.info(Color::BrightCyan)
		.error(Color::BrightRed)
		.warn(Color::BrightYellow);

	fern::Dispatch::new()
		.format(move |out, message, record| {
			let level = record.level();

			out.finish(format_args!(
				"{}: {}",
				colors.color(level).to_string().to_lowercase(),
				message
			))
		})
		.level(LevelFilter::Info)
		.chain(
			fern::Dispatch::new()
				.filter(|metadata| {
					!matches!(metadata.level(), Level::Error | Level::Warn)
				})
				.chain(std::io::stdout())
		)
		.chain(
			fern::Dispatch::new()
				.level(log::LevelFilter::Error)
				.level(log::LevelFilter::Warn)
				.chain(std::io::stderr())
		)
		.apply()
		.ok();
}
