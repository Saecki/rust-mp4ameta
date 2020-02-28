//! A library to read iTunes style MPEG-4 audio metadata

pub use crate::atom::Atom;
pub use crate::content::{Content, Data};
pub use crate::error::{Error, ErrorKind, Result};
pub use crate::tag::Tag;

mod atom;
mod content;
mod error;
mod tag;
