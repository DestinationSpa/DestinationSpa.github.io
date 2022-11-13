use {
    markdown_it::{plugins::cmark, MarkdownIt},
    tagger::no_attr,
};

use {super::*, crate::Renderer};

impl Website {
    pub fn render(&self, r: &mut Renderer) {
        r.put_raw_escapable(r#"<?xml version="1.0" encoding="UTF-8"?>"#);

        r.elem("html", |r| {
            r.attr("xmlns", "https://www.w3.org/1999/xhtml");
            r.attr("lang", "fr");
        })
        .build(|r| {
            r.elem("body", no_attr()).build(|r| {
                self.home.render(r);
                self.introduction.render(r);
                self.images.render(r);
                self.contacts.render(r);
                self.hours.render(r);
                self.benefits.render(r);
            });
        });
    }
}

impl Home {
    fn render(&self, r: &mut Renderer) {
        self.title.render(r, 1);
        self.subtitle.render(r, 2);
        self.note.render(r);
    }
}

impl Introduction {
    fn render(&self, r: &mut Renderer) {
        self.title.render(r, 3);
        self.text.render(r);
    }
}

impl Images {
    fn render(&self, r: &mut Renderer) {
        self.title.render(r, 3);
        self.note.render(r);
        for image in &self.images {
            image.render(r, true);
        }
    }
}

impl Contacts {
    fn render(&self, r: &mut Renderer) {
        self.title.render(r, 3);
        self.note.render(r);
        self.phone.render(r);
        self.email.render(r);
        self.location.render(r);
        self.socials.render(r);
    }
}

impl Location {
    fn render(&self, r: &mut Renderer) {}
}

impl Socials {
    fn render(&self, r: &mut Renderer) {
        self.facebook.render(r);
        self.instagram.render(r);
    }
}

impl Social {
    fn render(&self, r: &mut Renderer) {}
}

impl Hours {
    fn render(&self, r: &mut Renderer) {
        self.title.render(r, 3);
        self.note.render(r);
        self.monday.render(r);
        self.tuesday.render(r);
        self.wednesday.render(r);
        self.thursday.render(r);
        self.friday.render(r);
        self.saturday.render(r);
        self.sunday.render(r);
    }
}

impl Hour {
    fn render(&self, r: &mut Renderer) {}
}

impl Benefits {
    fn render(&self, r: &mut Renderer) {
        self.title.render(r, 3);
        for category in &self.categories {
            category.render(r);
        }
    }
}

impl Category {
    fn render(&self, r: &mut Renderer) {
        self.title.render(r, 4);
        self.image.render(r, false);
        self.description.render(r);
        for benefit in &self.benefits {
            benefit.render(r);
        }
    }
}

impl Benefit {
    fn render(&self, r: &mut Renderer) {
        self.title.render(r, 5);
        self.price.render(r);
        self.book.render(r);
        self.description.render(r);
    }
}

impl Book {
    fn render(&self, r: &mut Renderer) {}
}

impl Email {
    fn render(&self, r: &mut Renderer) {}
}

impl Image {
    fn render(&self, r: &mut Renderer, caption: bool) {
        r.elem("figure", no_attr()).build(|r| {
            r.single("img", |r| {
                r.attr("src", format!("res/images/{}", self.0 .0));
                r.attr("alt", &self.0 .1);
            });

            r.elem("figcaption", no_attr()).build(|r| {
                r.put_raw(&self.0 .1);
            });
        });
    }
}

impl Inline {
    fn render(&self, r: &mut Renderer) {
        r.put_raw_escapable(parse_md(false, &self.0));
    }
}

impl Note {
    fn render(&self, r: &mut Renderer) {
        if let Some(inline) = &self.0 {
            r.elem("p", |r| {
                r.attr("class", "note");
            })
            .build(|r| {
                inline.render(r);
            });
        }
    }
}

impl Phone {
    fn render(&self, r: &mut Renderer) {}
}

impl Price {
    fn render(&self, r: &mut Renderer) {}
}

impl Text {
    fn render(&self, r: &mut Renderer) {
        r.put_raw_escapable(parse_md(true, &self.0));
    }
}

impl Title {
    fn render(&self, r: &mut Renderer, level: u8) {
        r.elem(format!("h{}", level), no_attr()).build(|r| {
            self.0.render(r);
        });
    }
}

fn parse_md(block: bool, input: &str) -> String {
    let parser = &mut MarkdownIt::new();

    if block {
        use cmark::block::*;
        paragraph::add(parser);
    }

    use cmark::inline::*;
    autolink::add(parser);
    backticks::add(parser);
    emphasis::add(parser);
    entity::add(parser);
    escape::add(parser);
    link::add(parser);

    parser
        .parse(input)
        .xrender()
        .trim()
        .replace("\r\n", " ")
        .replace('\n', " ")
}
