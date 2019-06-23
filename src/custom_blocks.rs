use super::blocks::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AllBlocks<'a> {
    #[serde(borrow)]
    TextBlock(TextBlock<'a>),
    Seconds(Seconds),
}

impl<'a> From<&'a AllBlocks<'a>> for Box<&'a dyn Block> {
    fn from(all_blocks: &'a AllBlocks<'a>) -> Self {
        Box::new(match all_blocks {
            AllBlocks::TextBlock(x) => x,
            AllBlocks::Seconds(x) => x,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextBlock<'a> {
    pub text: &'a str,
    pub text_color: Color,
}

impl<'a> Block for TextBlock<'a> {
    fn get_output(&self) -> BlockOutput {
        BlockOutput {
            name: "text_block",
            instance: 0,
            full_text: self.text.to_string(),
            color: self.text_color,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Seconds;

impl Block for Seconds {
    fn get_output(&self) -> BlockOutput {
        BlockOutput {
            name: "seconds",
            full_text: format!(
                "unix seconds: {}",
                std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            ..Default::default()
        }
    }
}
