use std::fs::File;
use std::io::prelude::*;

use lyrian::model::LyrianModel;

fn main() {
    // Read learning data.
    let mut f = File::open("./examples/sample_text.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    // Build model from text data.
    let mut model = LyrianModel::from_str(&*contents).unwrap();

    // Generate lyric.
    let lyric_1 = model.generate_lyric(7, true).unwrap();
    let lyric_2 = model.generate_lyric(7, false).unwrap();
    println!("syllable: {}", lyric_1.join());
    println!("mora    : {}", lyric_2.join());

    // Write json file.
    // let mut f_json = File::create("./examples/sample_model2.json").unwrap();
    // f_json.write_all(model.to_json_str().unwrap().as_bytes()).unwrap();
}
