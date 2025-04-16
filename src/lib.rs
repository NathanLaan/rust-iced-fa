//!
//! Rust-Iced-FA
//!
//! Wrapper for Font Awesome icons.
//!

use std::borrow::Cow;
use iced::advanced::graphics::text::font_system;
use iced::{Element, Font};
use iced::font::Family;
use iced::widget::text;


const FONT_DATA_FA_REGULAR: &[u8] =
    include_bytes!("../fonts/font-awesome-6-free-regular-400.otf");

const FONT_DATA_FA_BRANDS: &[u8] =
    include_bytes!("../fonts/font-awesome-6-brands-regular-400.otf");

const FONT_DATA_FA_SOLID: &[u8] =
    include_bytes!("../fonts/font-awesome-6-free-solid-900.otf");

///
/// Load Font Awesome files.
///
pub fn load_font_fontawesome() {
    let mut font_system = font_system().write().unwrap();
    font_system.load_font(Cow::from(FONT_DATA_FA_REGULAR));
    font_system.load_font(Cow::from(FONT_DATA_FA_BRANDS));
    font_system.load_font(Cow::from(FONT_DATA_FA_SOLID));
}

///
/// Unicode string for `https://fontawesome.com/icons/user`.
///
pub const FONTAWESOME_ICON_USER: &str = "f007";

///
/// Unicode string for `https://fontawesome.com/icons/1`.
///
pub const FA_ICON_0: &str = "30";
pub const FA_ICON_1: &str = "31";
pub const FA_ICON_2: &str = "32";
pub const FA_ICON_3: &str = "33";
pub const FA_ICON_4: &str = "34";
pub const FA_ICON_5: &str = "35";
pub const FA_ICON_6: &str = "36";
pub const FA_ICON_7: &str = "37";
pub const FA_ICON_8: &str = "38";
pub const FA_ICON_9: &str = "39";

///
/// The "Regular" version of Font Awesome version 6.
///
pub const FONT_FA_REGULAR: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    ..Font::DEFAULT
};


///
/// Create an iced `text` element containing the specified Font-Awesome icon.
///
/// Uses `FONT_FA_REGULAR`.
///
pub fn iced_text_icon<'a, Message>(code: &str) -> Element<'a, Message> {
    let code_u32 = u32::from_str_radix(&code, 16).unwrap();
    let unicode = char::from_u32(code_u32).unwrap();
    text(unicode).font(FONT_FA_REGULAR).into()
}
