//! Constructs for working with [semantic versions](https://semver.org/).

#![deny(missing_docs)]

use std::{
    fmt::{self, Display},
    str::FromStr,
};

use anyhow::{Context as _, Result, anyhow};
use serde::{Deserialize, Serialize, de::Error};

/// A [semantic version](https://semver.org/) number.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SemanticVersion {
    major: usize,
    minor: usize,
    patch: usize,
    rc: Option<usize>,
}

impl SemanticVersion {
    /// Returns a new [`SemanticVersion`] from the given components.
    pub const fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
            rc: None,
        }
    }

    /// Returns a new release candidate [`SemanticVersion`] from the given components.
    pub const fn with_rc(major: usize, minor: usize, patch: usize, rc: usize) -> Self {
        Self {
            major,
            minor,
            patch,
            rc: Some(rc),
        }
    }

    /// Returns the major version number.
    #[inline(always)]
    pub fn major(&self) -> usize {
        self.major
    }

    /// Returns the minor version number.
    #[inline(always)]
    pub fn minor(&self) -> usize {
        self.minor
    }

    /// Returns the patch version number.
    #[inline(always)]
    pub fn patch(&self) -> usize {
        self.patch
    }

    /// Returns the release candidate version number.
    #[inline(always)]
    pub fn rc(&self) -> Option<usize> {
        self.rc
    }
}

impl FromStr for SemanticVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rccomp = s.trim().split("-rc");
        let mut components = rccomp
            .next()
            .ok_or(anyhow!("missing version number"))?
            .trim()
            .split('.');
        let major = components
            .next()
            .context("missing major version number")?
            .parse()?;
        let minor = components
            .next()
            .context("missing minor version number")?
            .parse()?;
        let patch = components
            .next()
            .context("missing patch version number")?
            .parse()?;
        let rc = rccomp.next().and_then(|rc| rc.parse().ok());
        Ok(Self {
            major,
            minor,
            patch,
            rc,
        })
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(rc) = self.rc {
            write!(f, "{}.{}.{}-rc{}", self.major, self.minor, self.patch, rc)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}

impl Serialize for SemanticVersion {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SemanticVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Self::from_str(&string)
            .map_err(|_| Error::custom(format!("Invalid version string \"{string}\"")))
    }
}
