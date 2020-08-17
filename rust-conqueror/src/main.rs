use ferris_says::say;
use std::io::{stdout,BufWriter};

fn main() {
    let stdout = stdout();
    let msg = String::from("Hello fellow Rustaceans!");
    let width = msg.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(msg.as_bytes(),width,&mut writer).unwrap();
}
