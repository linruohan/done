fn main() -> anyhow::Result<()> {
	compile_gresources()?;

	#[cfg(windows)]
	compile_icon_winres()?;
	compile_icons()?;
	Ok(())
}

#[cfg(windows)]
fn compile_icon_winres() -> anyhow::Result<()> {
	use anyhow::Context;

	let mut res = winresource::WindowsResource::new();
	res.set("OriginalFileName", "mydone.exe");
	res.set_icon("./data/icons/mydone.ico");
	res
		.compile()
		.context("Failed to compile winresource resource")
}
fn compile_gresources() -> anyhow::Result<()> {
	glib_build_tools::compile_resources(
		&["data"],
		"data/resources.gresource.xml",
		"resources.gresource",
	);
	Ok(())
}

fn compile_icons() -> anyhow::Result<()> {
	relm4_icons_build::bundle_icons(
		// Name of the file that will be generated at `OUT_DIR`
		"icon_names.rs",
		// Optional app ID
		Some("com.github.linruohan.mydone"),
		// Custom base resource path:
		// * defaults to `/com/example/myapp` in this case if not specified explicitly
		// * or `/org/relm4` if app ID was not specified either
		None::<&str>,
		// Directory with custom icons (if any)
		None::<&str>,
		// List of icons to include
		[
			"menu",
			"loupe",
			"floppy",
			"computer",
			"star-filled-rounded",
			"dock-left",
			"task",
			"sonar",
			"check-round-outline2",
			"check-round-outline",
			"checkmark",
			"left",
			"warning-outline",
			"flag-outline-thin",
			"flag-outline-thick",
			"flag-filled",
			"work-week",
			"alarm",
			"plus",
			"arrow-circular-top-right",
			"cross-small-circle-filled",
			"info-outline",
			"editor",
			"pencil-and-paper-small",
			"update",
			"clipboard",
			"image-adjust-brightness",
			"file-cabinet",
			"star-outline-rounded",
			"controls",
			"dark-mode",
			"list-large",
			"horizontal-arrows",
			"size-vertically",
		],
	);
	Ok(())
}
