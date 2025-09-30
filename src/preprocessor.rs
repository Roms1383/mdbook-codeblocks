use std::collections::HashMap;

use log::error;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use serde::Deserialize;

use crate::language::{Language, SUPPORTED_LANGUAGES, SUPPORTED_OPTIONS};

#[derive(Debug, Default, Deserialize)]
pub struct Customization {
    pub icon: Option<String>,
    pub label: Option<String>,
    pub link: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Cfg {
    pub icon: Option<String>,
    pub overrides: HashMap<String, Customization>,
}

pub struct Codeblocks;

impl Codeblocks {
    pub fn new() -> Self {
        Self
    }

    /// preprocessing checks
    pub fn handle_preprocessing(&self) -> anyhow::Result<()> {
        use semver::{Version, VersionReq};
        use std::io::{stdin, stdout};

        let (ctx, book) = CmdPreprocessor::parse_input(stdin())?;
        let current = Version::parse(&ctx.mdbook_version)?;
        let built = VersionReq::parse(&format!("~{}", mdbook::MDBOOK_VERSION))?;

        if ctx.mdbook_version != mdbook::MDBOOK_VERSION && !built.matches(&current) {
            error!(
                "The {} plugin was built against version {} of mdbook, \
				      but we're being called from version {}, so may be incompatible.",
                self.name(),
                mdbook::MDBOOK_VERSION,
                ctx.mdbook_version
            );
        }
        let processed_book = self.run(&ctx, book)?;
        serde_json::to_writer(stdout(), &processed_book)?;
        Ok(())
    }
}

impl Preprocessor for Codeblocks {
    fn name(&self) -> &str {
        "codeblocks"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "html"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut config = Cfg {
            icon: None,
            overrides: HashMap::new(),
        };
        if let Some(cfg) = ctx.config.get_preprocessor(self.name()) {
            cfg.iter().for_each(|(key, val)| {
            if !SUPPORTED_OPTIONS.contains(&key.as_str()) { return; }
            match key.as_str() {
                key if val.is_table() => {
                  let mut customization: Customization = Customization { color: None, icon: None, label: None, link: None };
                  for (k, v) in val.as_table().unwrap() {
                    match k.as_str() {
                      "icon" if v.is_str() => {
                        customization.icon = v.as_str().map(ToString::to_string);
                      },
                      "color" if v.is_str() => {
                         customization.color = v.as_str().map(ToString::to_string);
                        },
                      "link" if v.is_str() => {
                         customization.link = v.as_str().map(ToString::to_string);
                        },
                      "label" if v.is_str() => {
                         customization.label = v.as_str().map(ToString::to_string);
                      },

                      _ => {
                        error!(
                          "got unexpected '{}' (expects 'rust', 'redscript', 'swift', 'lua', 'cpp' or 'icon')",
                          key,
                      );
                      std::process::exit(1)
                      },
                    };
                  }
                  config.overrides.insert(key.to_string(), customization);
                },
                "icon" if val.is_str() => {
                  config.icon = Some(val.as_str().unwrap().to_owned());
                },
                key => {
                  error!(
                    "got unexpected '{}' (expects 'rust', 'redscript', 'swift', 'lua', 'cpp' or 'icon')",
                    key,
                );
                std::process::exit(1)
                }
            }
        });
        }

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                let _ = process_code_blocks(chapter, &config)
                    .map(|s| {
                        chapter.content = s;
                    })
                    .map_err(|err| {
                        error!("{}", err);
                    });
            }
        });
        Ok(book)
    }
}

/// whether mark is a supported language or not
fn is_supported(mark: &str) -> bool {
    
    SUPPORTED_LANGUAGES
        .iter()
        .any(|language| language.as_mark().contains(&mark))
}

/// process prepending code blocks with vignettes
fn process_code_blocks(chapter: &mut Chapter, cfg: &Cfg) -> Result<String, std::fmt::Error> {
    use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag, TagEnd};
    use pulldown_cmark_to_cmark::cmark;

    enum State {
        None,
        Open,
        Gather,
    }

    let mut state = State::None;
    let mut buf = String::with_capacity(chapter.content.len());
    let parser = mdbook::utils::new_cmark_parser(&chapter.content, false);
    let events = parser.fold(vec![], |mut acc, ref e| {
        use CodeBlockKind::*;
        use CowStr::*;
        use Event::*;
        use State::*;
        match (e, &mut state) {
            (Start(Tag::CodeBlock(Fenced(Borrowed(mark)))), None) if is_supported(mark) => {
                let language = Language::from(*mark);
                acc.push(Start(Tag::Paragraph));
                acc.push(InlineHtml(open_vignette(language, cfg).into()));
                acc.push(HardBreak);
                acc.push(e.clone());
                state = Open;
            }
            (Text(Borrowed(_)), Open) => {
                acc.push(e.clone());
                state = Gather;
            }
            (Text(Borrowed(_)), Gather) => {
                acc.push(e.clone());
            }
            (End(TagEnd::CodeBlock), Gather) => {
                state = None;
                acc.push(e.clone());
                acc.push(HardBreak);
                acc.push(InlineHtml(close_vignette().into()));
                acc.push(End(TagEnd::Paragraph));
            }
            _ => {
                acc.push(e.clone());
            }
        };
        acc
    });
    cmark(events.iter(), &mut buf).map(|_| buf)
}

fn open_vignette(mark: Language, cfg: &Cfg) -> String {
    let link = mark.link(cfg);
    let name = mark.label(cfg);
    let icon = mark.icon(cfg);
    let color = mark
        .color(cfg)
        .map(|x| format!("--fa-primary-color:{x};color:{x};"))
        .unwrap_or("".into());
    format!(
        "<div class='codeblocks'>\n<a style=\"font-size:12px;text-decoration:none;{color}\" href=\"{link}\"><i style=\"font-size:18px;{color}\" class=\"fa fa-solid {icon}\"></i>&nbsp;&nbsp;{name}</a>",
    )
}

fn close_vignette() -> String {
    "</div>".to_string()
}
