#[derive(Debug)]
pub enum Error {
    Gettext(gettext::Error),
    Io(std::io::Error),
    DefaultNotFound,
}

impl From<gettext::Error> for Error {
    fn from(err: gettext::Error) -> Self {
        Self::Gettext(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
