use std::fmt;

#[derive(Debug)]
pub enum Error {
    InternalError(String),
    PluginNotFoundError(String),
    UIError(livi_external_ui::ui::LiviUIError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InternalError(msg) => write!(f, "Internal error: {msg}")?,
            Error::PluginNotFoundError(uri) => write!(f, "Plugin not found: {uri}")?,
            Error::UIError(e) => write!(f, "UI Error: {e}")?,
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::UIError(e) => Some(e),
            _ => None,
        }
    }
}


pub type Result<T, E = Error> = std::result::Result<T, E>;