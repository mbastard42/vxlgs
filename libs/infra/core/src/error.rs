#[derive(Debug)]
pub enum Error {
    Invariant(&'static str),
    External(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Invariant(msg) => write!(f, "invariant violated: {msg}"),
            Error::External(inner) => write!(f, "{inner}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Invariant(_) => None,
            Error::External(inner) => Some(&**inner),
        }
    }
}

impl Error {
    pub fn ext(e: impl std::error::Error + Send + Sync + 'static) -> Self {
        Error::External(Box::new(e))
    }
}

pub type Result<T> = core::result::Result<T, Error>; 

pub trait ResultExt<T> {
    fn corerr(self) -> Result<T>;
    fn cortext(self, msg: &'static str) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[inline]
    fn corerr(self) -> Result<T> {
        self.map_err(Error::ext)
    }

    #[inline]
    fn cortext(self, msg: &'static str) -> Result<T> {
        self.map_err(|_e| Error::Invariant(msg))
    }
}