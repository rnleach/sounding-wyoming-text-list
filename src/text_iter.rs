//! Breaks a str up into sections that represent a complete sounding, and provides an iterator
//! for them.

#[derive(Clone, Copy, Debug)]
pub struct SoundingText<'a> {
    pub upper_air: &'a str,
    pub stn_info_and_indexes: &'a str,
}

pub struct SoundingTextIterator<'a> {
    remaining: &'a str,
}

pub fn create_text_section_iterator(text: &str) -> SoundingTextIterator {
    SoundingTextIterator { remaining: text }
}

const PRE_START: &str = "<PRE>";
const PRE_END: &str = "</PRE>";

impl<'a> Iterator for SoundingTextIterator<'a> {
    type Item = SoundingText<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (upper_air, end) = find_next_pre_section(self.remaining)?;
        self.remaining = &self.remaining[end..];

        let (stn_info_and_indexes, end) = find_next_pre_section(self.remaining)?;
        self.remaining = &self.remaining[end..];

        Some(SoundingText {
            upper_air,
            stn_info_and_indexes,
        })
    }
}

fn find_next_pre_section(source: &str) -> Option<(&str, usize)> {
    let start = source.find(PRE_START)?;
    let end = (&source[start..]).find(PRE_END)?;

    Some((&source[(start + PRE_START.len())..end], end))
}
