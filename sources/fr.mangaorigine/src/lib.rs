#![no_std]
use aidoku::{Source, prelude::*};
use mangathemesia::{Impl, MangaThemesia, Params};

const BASE_URL: &str = "https://mangas-origines.fr";

struct MangaOrigine;

impl Impl for MangaOrigine {
	fn new() -> Self {
		Self
	}

	fn params(&self) -> Params {
		Params {
			base_url: BASE_URL.into(),
			manga_url_directory: "/catalogue".into(),
			date_format: "d MMMM yyyy".into(),
			date_locale: "fr_FR".into(),
			..Default::default()
		}
	}
}

register_source!(
	MangaThemesia<MangaOrigine>,
	Home,
	ImageRequestProvider,
	DeepLinkHandler
);
