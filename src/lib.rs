use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use svgbob::Settings;

pub struct Bob;

impl Bob {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Bob {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for Bob {
    fn name(&self) -> &str {
        "svgbob2"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // first load the configuration from the book.toml
        // also apply some default settings that are good for mdbook
        let mut settings = svgbob::Settings {
            background: "transparent".to_owned(),
            stroke_color: "var(--fg)".to_owned(),
            ..Default::default()
        };

        if let Some(cfg) = ctx.config.get_preprocessor(self.name()) {
            cfg.iter().for_each(|(key, val)| match key.as_str() {
                "font_size" => settings.font_size = val.clone().try_into().unwrap(),
                "font_family" => settings.font_family = val.clone().try_into().unwrap(),
                "fill_color" => settings.fill_color = val.clone().try_into().unwrap(),
                "background" => settings.background = val.clone().try_into().unwrap(),
                "stroke_color" => settings.stroke_color = val.clone().try_into().unwrap(),
                "stroke_width" => settings.stroke_width = val.clone().try_into().unwrap(),
                "scale" => settings.scale = val.clone().try_into().unwrap(),
                "enhance_circuitries" => {
                    settings.enhance_circuitries = val.clone().try_into().unwrap()
                }
                "include_backdrop" => settings.include_backdrop = val.clone().try_into().unwrap(),
                "include_styles" => settings.include_styles = val.clone().try_into().unwrap(),
                "include_defs" => settings.include_defs = val.clone().try_into().unwrap(),
                "merge_line_with_shapes" => {
                    settings.merge_line_with_shapes = val.clone().try_into().unwrap()
                }

                _ => (), // this should not happen
            });
        }

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                // saved to check if we are currently inside a codeblock
                let mut in_block = false;

                // if Windows crlf line endings are used, a code block will consist
                // of many different Text blocks, thus we need to buffer them in here
                // see https://github.com/raphlinus/pulldown-cmark/issues/507
                let mut diagram = String::new();
                let events = Parser::new_ext(&chapter.content, Options::all())
                    .map(|event| {
                        match (&event, in_block) {
                            // check if we are entering a svgbob codeblock
                            (
                                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(
                                    CowStr::Borrowed("svgbob"),
                                ))),
                                false,
                            ) => {
                                in_block = true;
                                diagram.clear();
                                None
                            }
                            // check if we are currently inside an svgbob block
                            (Event::Text(content), true) => {
                                diagram.push_str(content);
                                None
                            }
                            // check if we are exiting an svgbob block
                            (
                                Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(
                                    CowStr::Borrowed("svgbob"),
                                ))),
                                true,
                            ) => {
                                in_block = false;
                                Some(Event::Html(create_svg_html(&diagram, &settings).into()))
                            }
                            // if nothing matches, change nothing
                            _ => Some(event),
                        }
                    })
                    // flatten to fullfill the Trait boundaries of the cmark call below
                    .flatten();

                // create a buffer in which we can place the markdown
                let mut buf = String::with_capacity(chapter.content.len() + 128);

                // convert it back to markdown and replace the original chapter's content
                cmark(events, &mut buf, None).unwrap();
                chapter.content = buf;
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

fn create_svg_html(s: &str, settings: &Settings) -> String {
    let svg = svgbob::to_svg_with_settings(s, settings);

    // I am actually not sure why this has to be a pre tag
    // taken from https://github.com/badboy/mdbook-mermaid/blob/main/src/lib.rs
    format!("<pre class=\"svgbob\">{}</pre>", svg)
}
