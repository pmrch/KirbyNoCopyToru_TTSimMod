use compact_str::CompactString;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Name {
    pub english:  CompactString,
    pub japanese: CompactString,
}

#[derive(Deserialize)]
pub struct Image {
    pub file:   CompactString,
    pub url:    CompactString,
    pub height: u16,
    pub width:  u16,
}

#[derive(Deserialize)]
pub struct Card {
    pub image:     Image,
    pub name:      Name,
    pub abilities: Option<CompactString>,
    pub color:     CompactString,
    pub quantity:  CompactString,
    pub power:     u8,
}
