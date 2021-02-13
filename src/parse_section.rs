//! Parse a SoundingText into a (sounding_analysis::Sounding, HashMap<&'static str, f64>).
use crate::text_iter::SoundingText;
use sounding_analysis::Sounding;
use std::collections::HashMap;

pub fn parse(
    text: SoundingText,
    _source_description: &str,
) -> Option<(Sounding, HashMap<&'static str, f64>)> {
    println!("{}", text.upper_air );

    unimplemented!()
}
