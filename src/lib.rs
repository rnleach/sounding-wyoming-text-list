#![doc(test(attr(deny(warnings))))]
#![warn(missing_docs)]
//! Library to parse and iterate over weather soundings retrieved from [University of Wyoming][1].
//!
//!  [1]: http://weather.uwyo.edu/upperair/sounding.html

//
// API
//
pub use crate::error::Error;
pub use crate::sounding_iter::{parse_text, SoundingIterator};

//
// Internal use only.
//

mod error;
mod parse_section;
mod sounding_iter;
mod text_iter;
