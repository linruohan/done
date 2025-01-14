use i18n_embed::DesktopLanguageRequester;
use i18n_embed::{
	DefaultLocalizer, LanguageLoader, Localizer,
	fluent::{FluentLanguageLoader, fluent_language_loader},
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
	let loader: FluentLanguageLoader = fluent_language_loader!();

	loader
		.load_languages(&Localizations, &[loader.fallback_language().clone()])
		.unwrap();
	loader
});
#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        // i18n_embed_fl::fl!($crate::app::config::localization::LANGUAGE_LOADER, $message_id)
		""
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::app::config::localization::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}
// Get the `Localizer` to be used for localizing this library.
pub fn localizer() -> Box<dyn Localizer> {
	Box::new(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

pub fn init() {
	let localizer = super::localization::localizer();
	let requested_languages = DesktopLanguageRequester::requested_languages();

	if let Err(error) = localizer.select(&requested_languages) {
		eprintln!("Error while loading language: {error}");
	}
}
