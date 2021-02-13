//! Take in a string and produce an iterator of soundings.
use crate::text_iter::SoundingTextIterator;
use sounding_analysis::Sounding;
use std::collections::HashMap;

/// Iterator that produces soundings and the provider analyzed values as a HashMap.
pub struct SoundingIterator<'a> {
    text_iter: SoundingTextIterator<'a>,
    source_name: &'a str,
}

impl<'a> Iterator for SoundingIterator<'a> {
    type Item = (Sounding, HashMap<&'static str, f64>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let text = self.text_iter.next()?;
            if let res @ Some(_) = crate::parse_section::parse(text, self.source_name) {
                return res;
            }
        }
    }
}

/// Parse text into Sounding objects and their provider analyzed values as a HashMap.
pub fn parse_text<'a>(source_name: &'a str, text_data: &'a str) -> SoundingIterator<'a> {
    let text_iter = crate::text_iter::create_text_section_iterator(text_data);

    SoundingIterator {
        text_iter,
        source_name,
    }
}
