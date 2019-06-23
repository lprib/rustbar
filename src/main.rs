
use blocks::Block;
use blocks::ConfigFile;
use std::fs;
mod blocks;
mod custom_blocks;

static HEADER_STRING: &str = r##"{"version": 1} [["##;

fn main() {

    let file = fs::read_to_string("/home/liam/programming/rustbar/test.toml").unwrap();
    let a: ConfigFile = toml::from_str(&file).unwrap();

    run(a);
}

fn run<'a>(config: ConfigFile<'a>) -> ! {
    println!("{}", HEADER_STRING);

    let blocks_vec: Vec<Box<&dyn Block>> = config.blocks.iter().map(|x| x.into()).collect();

    loop {
        for (index, block) in blocks_vec.iter().enumerate() {
            println!("{}", serde_json::to_string(&block.get_output()).unwrap());
            if index != blocks_vec.len() - 1 {
                println!(",");
            }
        }
        println!("],[");
        std::thread::sleep_ms(1000);
    }
}
