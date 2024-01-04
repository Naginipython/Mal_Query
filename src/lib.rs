pub mod myanimelist;

#[cfg(feature = "jikan")]
pub mod jikan;

// TODO: tests for UserListBuilder
// TODO: implement updating tags, comments, start_date, finish_date
// TODO: test updating tags, comments, start_date, finish_date
// TODO: describe tags, comments, start_date, finish_date
// TODO: Launch 1.0

// TODO: feature: append_fields to builds, find intuitive way to add many fields, or Fields::All

// TODO: Manga model MalMangaData
// TODO: search_manga(?)
// TODO: get_manga(?)
// TODO: get_manga_rankings(?)
// TODO: get_manga_by_url(?)

// TODO: Various builders

// TODO: update manga
// TODO: delete manga
// TODO: get_user_mangalist

// -- Jikan --
// TODO: get_anime
// TODO: get_anime_characters
// TODO: get_anime_staff
// etc. https://docs.api.jikan.moe/