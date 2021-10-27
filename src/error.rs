/** A result that returns a highground error */
pub type IslandResult<T> = Result<T, IslandError>;

/** Represents a highground error */
#[derive(Clone)]
pub struct IslandError {
    reason: String,
}

impl IslandError {
    /** Construct a new highground error */
    pub fn new<T>(reason: T) -> Self
    where
        T: Into<String>,
    {
        return Self {
            reason: reason.into(),
        };
    }
}

impl std::fmt::Display for IslandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Highground: \"{}\"", self.reason)
    }
}

impl std::fmt::Debug for IslandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Highground: \"{}\"", self.reason)
    }
}

impl std::error::Error for IslandError {}
