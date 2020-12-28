use std::error::Error as StdError;
use std::{fmt, io};

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
}

impl StdError for Error {}

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
    fn as_ref(&self) -> &(dyn StdError + 'static) {
        self
    }
}

impl AsMut<dyn StdError> for Error {
    fn as_mut(&mut self) -> &mut (dyn StdError + 'static) {
        self
    }
}

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
    use std::io;

    #[test]
    fn test_errors() {
        assert_eq!(git::Error::branch_not_found("foo").to_string(), git::Error::BranchNotFound("foo".to_string()).to_string());
        assert_eq!("failed to find branch: foo", git::Error::BranchNotFound("foo".to_string()).to_string());
        assert_eq!("only fast-forward supported", git::Error::FastForwardOnly.to_string());

        let err = git::Error::from(git2::Error::new(git2::ErrorCode::Ambiguous, git2::ErrorClass::Checkout, "foo"));
        assert_eq!("foo; class=Checkout (20); code=Ambiguous (-5)", err.to_string());

        let mut err = git::Error::from(FuError::from(io::Error::new(io::ErrorKind::AlreadyExists, "foo")));
        assert_eq!("foo", err.to_string());
        assert_eq!("foo", err.as_ref().to_string());
        assert_eq!("foo", err.as_mut().to_string());

        assert_eq!("no message was found for commit", git::Error::NoMessageWasFound.to_string());
        assert_eq!("foo", git::Error::from(io::Error::new(io::ErrorKind::AlreadyExists, "foo")).to_string());
        assert_eq!(git::Error::repo_not_found("foo").to_string(), git::Error::RepoNotFound("foo".to_string()).to_string());
        assert_eq!("failed to find repo: foo", git::Error::RepoNotFound("foo".to_string()).to_string());
        assert_eq!("no url was set for the repo", git::Error::UrlNotSet.to_string());
    }
}
