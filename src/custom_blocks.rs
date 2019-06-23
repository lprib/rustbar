use super::blocks::*;
use serde::{Deserialize, Serialize};
use toml::value::Table;


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum CustomBlock<'a> {
    #[serde(borrow)]
    TextBlock(TextBlock<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextBlock<'a> {
    pub text: &'a str,
    pub text_color: Color,
}

// impl<'a> From<Table> for TextBlock<'a> {
//     fn from(table: Table) -> TextBlock<'a> {

//         TextBlock {
//             text: table.get("text").cloned().unwrap().as_str().unwrap(),
//             text_color: Color::from(table.get("text_color").cloned().unwrap().as_str().unwrap())
//         }
//     }
// }

impl<'a> Block for TextBlock<'a> {
    fn get_output(&self) -> BlockOutput {
        BlockOutput {
            name: "text_block",
            instance: 0,
            full_text: self.text,
            color: self.text_color,
            ..Default::default()
        }
    }
}
