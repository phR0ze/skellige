use std::{error::Error as StdError, fmt, io};

/// `Result<T>` provides a simplified result type with a common error type
pub type Result<T> = std::result::Result<T, Error>;

/// Define common error wrapper type
#[derive(Debug)]
pub enum Error {
    /// An error indicating that the given branch was not found.
    BranchNotFound(String),

    /// An error indicating that only fast forwards are allowed.
    FastForwardOnly,

    /// An error from fungus which might contain more errors
    Fungus(fungus::FuError),

    /// Git2 wrapped error
    Git2(git2::Error),

    /// An error indicating that no message was found.
    NoMessageWasFound,

    // Progress error occurred with indicatif
    Progress(io::Error),

    /// An error indicating that the given repo was not found.
    RepoNotFound(String),

    /// An error indicating that the URL was not set for the repo.
    UrlNotSet,
}

impl Error {
    /// Return an error indicating that the given branch was not found.
    pub fn branch_not_found<T: AsRef<str>>(pkg: T) -> Error {
        Error::BranchNotFound(pkg.as_ref().to_string())
    }

    /// Return an error indicating that the given repo was not found.
    pub fn repo_not_found<T: AsRef<str>>(repo: T) -> Error {
        Error::RepoNotFound(repo.as_ref().to_string())
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn is<T: StdError+'static>(&self) -> bool {
        self.as_ref().is::<T>()
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn downcast_ref<T: StdError+'static>(&self) -> Option<&T> {
        self.as_ref().downcast_ref::<T>()
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn downcast_mut<T: StdError+'static>(&mut self) -> Option<&mut T> {
        self.as_mut().downcast_mut::<T>()
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn source(&self) -> Option<&(dyn StdError+'static)> {
        self.as_ref().source()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BranchNotFound(ref pkg) => write!(f, "failed to find branch: {}", pkg),
            Error::FastForwardOnly => write!(f, "only fast-forward supported"),
            Error::Fungus(ref err) => write!(f, "{}", err),
            Error::Git2(ref err) => write!(f, "{}", err),
            Error::NoMessageWasFound => write!(f, "no message was found for commit"),
            Error::RepoNotFound(ref repo) => write!(f, "failed to find repo: {}", repo),
            Error::Progress(ref err) => write!(f, "{}", err),
            Error::UrlNotSet => write!(f, "no url was set for the repo"),
        }
    }
}

impl AsRef<dyn StdError> for Error {
    fn as_ref(&self) -> &(dyn StdError+'static) {
        match *self {
            Error::BranchNotFound(_) => self,
            Error::FastForwardOnly => self,
            // Unwrap a fungus error so it is transparent
            Error::Fungus(ref err) => err.as_ref(),
            Error::Git2(ref err) => err,
            Error::NoMessageWasFound => self,
            Error::RepoNotFound(_) => self,
            Error::Progress(ref err) => err,
            Error::UrlNotSet => self,
        }
    }
}

impl AsMut<dyn StdError> for Error {
    fn as_mut(&mut self) -> &mut (dyn StdError+'static) {
        match *self {
            Error::BranchNotFound(_) => self,
            Error::FastForwardOnly => self,
            // Unwrap a fungus error so it is transparent
            Error::Fungus(ref mut err) => err.as_mut(),
            Error::Git2(ref mut err) => err,
            Error::NoMessageWasFound => self,
            Error::RepoNotFound(_) => self,
            Error::Progress(ref mut err) => err,
            Error::UrlNotSet => self,
        }
    }
}

impl StdError for Error {}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::Git2(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Progress(err)
    }
}

impl From<fungus::FuError> for Error {
    fn from(err: fungus::FuError) -> Error {
        Error::Fungus(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_errors() {
        // Error::BranchNotFound(String)
        let mut err = git::Error::BranchNotFound("foo".to_string());
        assert_eq!(git::Error::branch_not_found("foo").to_string(), err.to_string());
        assert_eq!("failed to find branch: foo", err.to_string());
        assert_eq!("failed to find branch: foo", err.as_ref().to_string());
        assert_eq!("failed to find branch: foo", err.as_mut().to_string());
        assert!(err.downcast_ref::<git::Error>().is_some());
        assert!(err.downcast_mut::<git::Error>().is_some());
        assert!(err.source().is_none());

        // FastForwardOnly,
        let mut err = git::Error::FastForwardOnly;
        assert_eq!("only fast-forward supported", err.to_string());
        assert_eq!("only fast-forward supported", err.to_string());
        assert_eq!("only fast-forward supported", err.as_ref().to_string());
        assert_eq!("only fast-forward supported", err.as_mut().to_string());
        assert!(err.downcast_ref::<git::Error>().is_some());
        assert!(err.downcast_mut::<git::Error>().is_some());
        assert!(err.source().is_none());

        // Fungus(fungus::FuError),
        let mut err = git::Error::from(FuError::from(FileError::FailedToExtractString));
        assert_eq!("failed to extract string from file", err.to_string());
        assert_eq!("failed to extract string from file", err.as_ref().to_string());
        assert_eq!("failed to extract string from file", err.as_mut().to_string());
        assert!(err.downcast_ref::<FileError>().is_some());
        assert!(err.downcast_mut::<FileError>().is_some());
        assert!(err.source().is_none());

        // Git2(git2::Error),
        let mut err = git::Error::from(git2::Error::new(git2::ErrorCode::Ambiguous, git2::ErrorClass::Checkout, "foo"));
        assert_eq!("foo; class=Checkout (20); code=Ambiguous (-5)", err.to_string());
        assert_eq!("foo; class=Checkout (20); code=Ambiguous (-5)", err.as_ref().to_string());
        assert_eq!("foo; class=Checkout (20); code=Ambiguous (-5)", err.as_mut().to_string());
        assert!(err.downcast_ref::<git2::Error>().is_some());
        assert!(err.downcast_mut::<git2::Error>().is_some());
        assert!(err.source().is_none());

        // NoMessageWasFound,
        let mut err = git::Error::NoMessageWasFound;
        assert_eq!("no message was found for commit", err.to_string());
        assert_eq!("no message was found for commit", err.as_ref().to_string());
        assert_eq!("no message was found for commit", err.as_mut().to_string());
        assert!(err.downcast_ref::<git::Error>().is_some());
        assert!(err.downcast_mut::<git::Error>().is_some());
        assert!(err.source().is_none());

        // Progress(io::Error),
        let mut err = git::Error::from(io::Error::new(io::ErrorKind::AlreadyExists, "foo"));
        assert_eq!("foo", err.to_string());
        assert_eq!("foo", err.as_ref().to_string());
        assert_eq!("foo", err.as_mut().to_string());
        assert!(err.downcast_ref::<io::Error>().is_some());
        assert!(err.downcast_mut::<io::Error>().is_some());
        assert!(err.source().is_none());

        // RepoNotFound(String),
        let mut err = git::Error::RepoNotFound("foo".to_string());
        assert_eq!(git::Error::repo_not_found("foo").to_string(), err.to_string());
        assert_eq!("failed to find repo: foo", err.to_string());
        assert_eq!("failed to find repo: foo", err.as_ref().to_string());
        assert_eq!("failed to find repo: foo", err.as_mut().to_string());
        assert!(err.downcast_ref::<git::Error>().is_some());
        assert!(err.downcast_mut::<git::Error>().is_some());
        assert!(err.source().is_none());

        // UrlNotSet,
        let mut err = git::Error::UrlNotSet;
        assert_eq!("no url was set for the repo", err.to_string());
        assert_eq!("no url was set for the repo", err.to_string());
        assert_eq!("no url was set for the repo", err.as_ref().to_string());
        assert_eq!("no url was set for the repo", err.as_mut().to_string());
        assert!(err.downcast_ref::<git::Error>().is_some());
        assert!(err.downcast_mut::<git::Error>().is_some());
        assert!(err.source().is_none());
    }
}
