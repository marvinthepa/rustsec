//! Rust platforms

mod platforms;

#[cfg(feature = "std")]
mod req;
mod tier;

pub use self::{tier::Tier, platforms::*};

#[cfg(feature = "std")]
pub use self::req::PlatformReq;

use crate::target::*;
use core::fmt;

/// Rust platforms supported by mainline rustc
///
/// Sourced from <https://doc.rust-lang.org/nightly/rustc/platform-support.html>
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Platform {
    /// "Target triple" string uniquely identifying the platform. See:
    /// <https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md>
    ///
    /// These are defined in the `rustc_target` crate of the Rust compiler:
    /// <https://github.com/rust-lang/rust/blob/master/src/librustc_target/spec/mod.rs>
    pub target_triple: &'static str,

    /// Target architecture `cfg` attribute (i.e. `cfg(target_arch)`)
    pub target_arch: Arch,

    /// Target OS `cfg` attribute (i.e. `cfg(target_os)`).
    pub target_os: OS,

    /// Target environment `cfg` attribute (i.e. `cfg(target_env)`).
    /// Only used when needed for disambiguation, e.g. on many GNU platforms
    /// this value will be `None`.
    pub target_env: Option<Env>,

    /// Tier of this platform:
    ///
    /// - `Tier::One`: guaranteed to work
    /// - `Tier::Two`: guaranteed to build
    /// - `Tier::Three`: unofficially supported with no guarantees
    pub tier: Tier,
}

impl Platform {
    /// All valid Rust platforms usable from the mainline compiler
    pub const ALL: &'static [Platform] = ALL;

    /// Find a Rust platform by its "target triple", e.g. `i686-apple-darwin`
    pub fn find(target_triple: &str) -> Option<&'static Platform> {
        Self::ALL
            .iter()
            .find(|platform| platform.target_triple == target_triple)
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.target_triple)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::Platform;
    use std::collections::HashSet;

    /// Ensure there are no duplicate target triples in the platforms list
    #[test]
    fn no_dupes_test() {
        let mut target_triples = HashSet::new();

        for platform in Platform::ALL {
            assert!(
                target_triples.insert(platform.target_triple),
                "duplicate target triple: {}",
                platform.target_triple
            );
        }
    }
}
