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

const PRE_START: &str = "<PRE>";
const PRE_START_LC: &str = "<pre>";
const PRE_END: &str = "</PRE>";
const PRE_END_LC: &str = "</pre>";

fn find_next_pre_section(source: &str) -> Option<(&str, usize)> {
    let start = source
        .find(PRE_START)
        .or_else(|| source.find(PRE_START_LC))?;
    let end = source[start..]
        .find(PRE_END)
        .or_else(|| source[start..].find(PRE_END_LC))?
        + start;

    let chunk = &source[(start + PRE_START.len())..end];

    Some((chunk, end))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_otx_count() {
        let text = get_otx();

        let iter = create_text_section_iterator(text);
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn test_otx2_count() {
        let text = get_otx2();

        let iter = create_text_section_iterator(text);
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn test_tfx() {
        let text = get_tfx();

        let iter = create_text_section_iterator(text);
        assert_eq!(iter.count(), 20);
    }
}
