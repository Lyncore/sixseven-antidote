use std::path::Path;
use std::sync::OnceLock;

mod docx;
mod txt;

pub use docx::DocxHandler;
pub use txt::TxtHandler;

pub trait FormatHandler: Send + Sync {
    fn extension(&self) -> &str;
    fn count_matches(&self, path: &Path) -> Result<usize, String>;
    fn replace(&self, path: &Path) -> Result<usize, String>;
}

pub struct Registry(Vec<Box<dyn FormatHandler>>);

impl Registry {
    fn new() -> Self {
        Self(vec![Box::new(TxtHandler), Box::new(DocxHandler)])
    }

    pub fn get(&self, ext: &str) -> Option<&dyn FormatHandler> {
        self.0
            .iter()
            .find(|h| h.extension() == ext)
            .map(|h| h.as_ref())
    }

    pub fn extensions(&self) -> Vec<&str> {
        self.0.iter().map(|h| h.extension()).collect()
    }
}

static REGISTRY: OnceLock<Registry> = OnceLock::new();

pub fn registry() -> &'static Registry {
    REGISTRY.get_or_init(Registry::new)
}
