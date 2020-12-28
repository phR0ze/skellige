mod error;
mod git;

/// All essential symbols in a simple consumable way
///
/// ### Examples
/// ```
/// use fungus::prelude::*;
/// ```
pub mod prelude {
    // Export all types inside the git module for namespace clarity
    pub mod git {
        pub use crate::error::Error;
        pub use crate::error::Result;
        pub use crate::git::*;
    }

    // Re-export fungus
    pub use fungus::prelude::*;
}
