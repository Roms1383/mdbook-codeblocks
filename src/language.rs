use crate::preprocessor::Cfg;
use log::warn;

pub const SUPPORTED_LANGUAGES: [Language; 5] = [
    Language::Redscript,
    Language::Swift,
    Language::Cpp,
    Language::Lua,
    Language::Rust,
];

pub const SUPPORTED_OPTIONS: [&'static str; 5] = [&"rust", &"redscript", &"lua", &"cpp", &"swift"];

#[derive(Clone, Copy, PartialEq)]
pub enum Language {
    Empty,
    Unknown,
    Swift,
    Redscript,
    Rust,
    Cpp,
    Lua,
}

impl Language {
    pub fn as_mark(&self) -> &str {
        match self {
            Self::Empty | Self::Unknown => "",
            Self::Redscript => "swift reds",
            Self::Swift => "swift",
            Self::Lua => "lua",
            Self::Cpp => "cpp",
            Self::Rust => "rust",
        }
    }
    pub fn as_option(&self) -> Option<&str> {
        match self {
            Self::Empty | Self::Unknown => None,
            Self::Redscript => Some("redscript"),
            lang => Some(lang.as_mark()),
        }
    }
    pub fn as_official_mark(&self) -> &str {
        match self {
            Self::Redscript => Self::Swift.as_mark(),
            s => s.as_mark(),
        }
    }
    pub fn label<'a>(&'a self, cfg: &'a Cfg) -> &'a str {
        if let Some(option) = cfg.overrides.get(self.as_option().unwrap()) {
            if let Some(ref label) = option.label {
                return label.as_str();
            }
        }
        self.as_ref()
    }
    pub fn link<'a>(&'a self, cfg: &'a Cfg) -> &'a str {
        if let Some(option) = cfg.overrides.get(self.as_option().unwrap()) {
            if let Some(ref link) = option.link {
                return link.as_str();
            }
        }
        match self {
            Self::Unknown | Self::Empty => "#",
            Self::Swift => "https://developer.apple.com/swift",
            Self::Redscript => "https://wiki.redmodding.org/redscript",
            Self::Rust => "https://www.rust-lang.org",
            Self::Cpp => "https://isocpp.org",
            Self::Lua => "https://www.lua.org",
        }
    }
    pub fn icon<'a>(&'a self, cfg: &'a Cfg) -> &'a str {
        if let Some(option) = cfg.overrides.get(self.as_option().unwrap()) {
            if let Some(ref icon) = option.icon {
                return icon.as_str();
            }
        }
        match self {
            Self::Empty | Self::Unknown => "",
            Self::Swift => "fa-dove",
            Self::Redscript => "fa-r",
            Self::Rust => "fa-spaghetti-monster-flying",
            Self::Cpp => "fa-cube",
            Self::Lua => "fa-globe",
        }
    }
    pub fn color<'a>(&'a self, cfg: &'a Cfg) -> Option<&'a str> {
        if let Some(option) = cfg.overrides.get(self.as_option().unwrap()) {
            if let Some(ref color) = option.color {
                if let Err(_) = raster::Color::hex(color) {
                    if let Err(_) = color_name::Color::val().by_string(color.clone()) {
                        warn!("unknown color '{color}', skipped...");
                        return None;
                    }
                }
                return Some(color.as_str());
            }
        }
        None
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            return Language::Empty;
        }
        SUPPORTED_LANGUAGES
            .iter()
            .copied()
            .find(|language| language.as_mark() == value)
            .unwrap_or(Language::Unknown)
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Self::Empty => "",
            Self::Unknown => "unknown",
            Self::Swift => "Swift",
            Self::Redscript => "Redscript",
            Self::Rust => "Rust",
            Self::Cpp => "C++",
            Self::Lua => "Lua",
        }
    }
}
