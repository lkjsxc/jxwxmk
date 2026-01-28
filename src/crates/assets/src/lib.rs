use rust_embed::RustEmbed;
use mime_guess::from_path;

#[derive(RustEmbed)]
#[folder = "../../static"]
struct Asset;

pub fn get_asset(path: &str) -> Option<(std::borrow::Cow<'static, [u8]>, String)> {
    let file = Asset::get(path)?;
    let mime = from_path(path).first_or_octet_stream().to_string();
    Some((file.data, mime))
}

pub fn get_index() -> Option<(std::borrow::Cow<'static, [u8]>, String)> {
    get_asset("index.html")
}

pub fn init() {}
