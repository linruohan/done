#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
  )]
use anyhow::Result;
use app::config::{info::APP_ID, setup};
use relm4::RelmApp;

use app::Done;

mod app;

fn main() -> Result<()> {
	let app = RelmApp::new(APP_ID);
	setup::init()?;
	app.run_async::<Done>(());
	Ok(())
}
