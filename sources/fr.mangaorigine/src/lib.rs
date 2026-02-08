#![no_std]
use aidoku::{Source, prelude::*, MangaStatus};
use madara::{Impl, Madara, Params, LoadMoreStrategy};
use aidoku::alloc::String;

// URL de base pour MangaOrigine - Vérifier que c'est bien l'URL correcte
const BASE_URL: &str = "https://mangas-origines.fr";

struct MangaOrigine;

impl Impl for MangaOrigine {
	fn new() -> Self {
		Self
	}

	fn params(&self) -> Params {
		Params {
			base_url: BASE_URL.into(),
			// Chemin pour les URLs des mangas sur MangaOrigine
			source_path: "oeuvre".into(),
			use_new_chapter_endpoint: false,
			use_style_images: false,
			use_load_more_request: LoadMoreStrategy::Never,
			filter_non_manga_items: true,
			default_viewer: aidoku::Viewer::Default,
			datetime_format: "d MMMM yyyy".into(),
			datetime_locale: "fr_FR".into(),
			datetime_timezone: "Europe/Paris".into(),
			// Endpoint pour la page des genres
			genre_endpoint: "/catalogue".into(),
			search_page: |page| {
				if page == 1 {
					"".into()
				} else {
					format!("page/{}/", page).into()
				}
			},
			// Sélecteurs spécifiques à MangaOrigine
			search_manga_selector: "div.page-item-detail.manga".into(),
			search_manga_url_selector: "div.post-title a".into(),
			search_manga_title_selector: "div.post-title a".into(),
			search_manga_cover_selector: "img".into(),
			// Details selectors - spécifiques à MangaOrigine
			details_title_selector: "div.post-title h3, div.post-title h1".into(),
			details_cover_selector: "div.summary_image img".into(),
			details_author_selector: "div.author-content > a".into(),
			details_artist_selector: "div.artist-content > a".into(),
			details_description_selector: "div.summary_content".into(),
			details_tag_selector: "div.genres-content > a".into(),
			details_status_selector: "div.post-content_item:contains('État') span.summary-content".into(),
			details_type_selector: "".into(),
			// Chapter selectors - spécifiques à MangaOrigine
			chapter_selector: "ul.main li.wp-manga-chapter".into(),
			chapter_url_selector: "a".into(),
			chapter_title_selector: "a".into(),
			chapter_date_selector: "span.chapter-release-date".into(),
			chapter_thumbnail_selector: "".into(),
			// Page list selector
			page_list_selector: "div.reading-content img".into(),
			// Désactiver les protecteurs de chapitre
			chapter_protector_selector: "".into(),
			chapter_protector_password_prefix: "".into(),
			chapter_protector_data_prefix: "".into(),
		}
	}
}

fn status_mapping(status: String) -> MangaStatus {
	match status.to_lowercase().as_str() {
		"ongoing" | "en cours" | "en cours de publication" => MangaStatus::Ongoing,
		"completed" | "terminé" | "fini" | "complet" => MangaStatus::Completed,
		"paused" | "en pause" | "hiatus" => MangaStatus::Hiatus,
		"cancelled" | "annulé" => MangaStatus::Cancelled,
		_ => MangaStatus::Unknown,
	}
}

register_source!(
	Madara<MangaOrigine>,
	Home,
	ImageRequestProvider,
	DeepLinkHandler
);
