use minimal_app::MinimalApp;
use oml_game::Game;
use tracing_subscriber::FmtSubscriber;

pub fn main() -> anyhow::Result<()> {
	let use_ansi = atty::is(atty::Stream::Stdout);

	let subscriber = FmtSubscriber::builder()
		.with_max_level(tracing::Level::TRACE)
		.with_ansi(use_ansi) // sublime console doesn't like it :(
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

	let app = MinimalApp::default();

	match Game::run(app) {
		Ok(_) => {},
		Err(e) => {
			tracing::error!("Game returned {}", &e)
		},
	}

	Ok(())
}

mod minimal_app;
