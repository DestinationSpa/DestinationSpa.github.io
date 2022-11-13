mod schema;

use std::fs::{read_to_string, File};

use tagger::{upgrade_write, Adaptor};

use schema::Website;

type Renderer = tagger::ElemWriter<Adaptor<File>>;

fn main() {
    let input = read_to_string("res/content.ron").unwrap();
    let website: Website = ron::from_str(&input).unwrap();
    let output = File::create("index.xhtml").unwrap();
    let renderer = &mut tagger::new(upgrade_write(output));
    website.render(renderer);
}
