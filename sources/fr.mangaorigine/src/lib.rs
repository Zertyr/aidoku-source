#![no_std]
use aidoku::{Source, prelude::*, aidoku::MangaStatus};
use madara::{Impl, Madara, Params, LoadMoreStrategy};

const BASE_URL: &str = "https://mangas-origines.fr";

struct MangaOrigine;

impl Impl for MangaOrigine {
	fn new() -> Self {
		Self
	}

	fn params(&self) -> Params {
		Params {
			base_url: BASE_URL.into(),
			source_path: "/manga".into(),
			use_new_chapter_endpoint: false,
			use_style_images: false,
			use_load_more_request: LoadMoreStrategy::AutoDetect,
			filter_non_manga_items: true,
			default_viewer: aidoku::Viewer::Default,
			datetime_format: "d MMMM yyyy".into(),
			datetime_locale: "fr_FR".into(),
			datetime_timezone: "Europe/Paris".into(),
			genre_endpoint: "/catalogue".into(),
			search_page: |page| format!("/page/{}", page),
			search_manga_selector: "//div[@class='bs']",
			search_manga_url_selector: ".//div[@class='bsx']//a",
			search_manga_title_selector: ".//div[@class='tt']//a",
			search_manga_cover_selector: ".//div[@class='limit']//img",
			search_manga_cover_attr: "src",
			search_manga_author_selector: ".//div[@class='tsinfo']//i[contains(@class,'author')]/following-sibling::a",
			search_manga_latest_chapter_selector: ".//div[@class='epxs']//a",
			search_manga_rating_selector: None,
			search_manga_tags_selector: ".//div[@class='mgen']//a",
			search_manga_status_selector: ".//div[@class='tsinfo']//i[contains(@class,'status')]/following-sibling::span",
			search_manga_status_mapping: status_mapping,
			search_manga_views_selector: None,
			search_manga_year_selector: None,
			search_manga_no_data: |e| e.select(".//div[@class='bsx']//a").is_empty(),
			search_manga_parse_id: |url| url.split('/').last().unwrap().to_string(),
			search_manga_parse_status: |status| status.to_lowercase(),
			search_manga_parse_rating: |_| 0.0,
			search_manga_parse_views: |_| 0,
			search_manga_parse_year: |_| 0,
		}
	}
}

fn status_mapping(status: String) -> MangaStatus {
	match status.to_lowercase().as_str() {
		"ongoing" | "en cours" => MangaStatus::Ongoing,
		"completed" | "terminé" | "fini" => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	}
}

register_source!(
	Madara<MangaOrigine>,
	Home,
	ImageRequestProvider,
	DeepLinkHandler
);
