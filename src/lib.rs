mod error;
mod git;

// Re-exports
pub use fungus;
pub use git2;

/// All essential symbols in a simple consumable way
///
/// ### Examples
/// ```
/// use skellige::prelude::*;
/// ```
pub mod prelude {
    // Export all types inside the git module for namespace clarity
    pub mod git {
        pub use crate::error::Error;
        pub use crate::error::Result;
        pub use crate::git::*;
    }

    // Re-exports
    pub use fungus::prelude::*;
}
