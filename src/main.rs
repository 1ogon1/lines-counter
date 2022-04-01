mod reader;
mod settings;
mod utils;

use reader::Reader;

fn main() {
    Reader::new().read();
}
