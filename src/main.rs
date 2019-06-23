use blocks::Block;
use serde_json::json;
use std::fs;

mod blocks;
mod custom_blocks;

use blocks::*;
use custom_blocks::*;

fn main() {
    // let c = ConfigFile {
    //     blocks: vec![CustomBlock::TextBlock(TextBlock {
    //         text: "henlo",
    //         text_color: Color(6, 7, 8),
    //     })],
    // };
    let c = CustomBlock::TextBlock(TextBlock {
        text: "henlo",
        text_color: Color(6, 7, 8),
    });

    let file = fs::read_to_string("/home/liam/programming/rustbar/test.toml").unwrap();
    let a: ConfigFile = toml::from_str(&file).unwrap();

    println!("{:#?}", a);
}
