use serde::{Deserialize, Serialize, Serializer};
use toml::value::Table;
use super::custom_blocks::CustomBlock;

#[derive(Copy, Clone, Deserialize, Debug)]
#[serde(from = "&str")]
pub struct Color(pub u8, pub u8, pub u8);

impl Serialize for Color {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2))
    }
}

impl Default for Color {
    fn default() -> Self {
        Color(0, 0, 0)
    }
}

//todo make an actual deserializer with better error messages instead of this hack
impl<'a> From<&'a str> for Color {
    fn from(s: &'a str) -> Color {
        assert!(
            s.chars().next() == Some('#'),
            "expected # at start of color definition"
        );
        let r = u8::from_str_radix(&s[1..3], 16)
            .expect("expected 6 valid hex chars after # in color definition");
        let g = u8::from_str_radix(&s[3..5], 16)
            .expect("expected 6 valid hex chars after # in color definition");
        let b = u8::from_str_radix(&s[5..7], 16)
            .expect("expected 6 valid hex chars after # in color definition");
        Color(r, g, b)
    }
}

pub enum MinWidth<'a> {
    Pixels(u32),
    StringLength(&'a str),
}

impl<'a> Default for MinWidth<'a> {
    fn default() -> Self {
        MinWidth::Pixels(5)
    }
}

impl<'a> Serialize for MinWidth<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self {
            MinWidth::Pixels(p) => serializer.serialize_u32(*p),
            MinWidth::StringLength(s) => serializer.serialize_str(s),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Align {
    Center,
    Left,
    Right,
}

impl Default for Align {
    fn default() -> Self {
        Align::Center
    }
}

#[derive(Default, Serialize)]
pub struct BlockOutput<'a> {
    pub name: &'a str,
    pub instance: u32,
    pub full_text: &'a str,
    pub color: Color,
    pub background_color: Color,
    pub border_color: Color,
    pub min_width: MinWidth<'a>,
    pub align: Align,
    pub urgent: bool,
    pub separator: bool,
    pub separator_block_width: u32,
    pub markup: bool,
}

pub trait Block/*: From<Table>*/ {
    fn get_output(&self) -> BlockOutput;
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile<'a> {
    #[serde(borrow)]
    pub blocks: Vec<CustomBlock<'a>>,
}
