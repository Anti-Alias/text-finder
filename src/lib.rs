use std::io::Read;
use std::fmt;

use slice_ring_buffer::SliceRingBuffer;

/// String type where each character is stored as 4 bytes in a Vec.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CharString(Vec<char>);
impl CharString {
    pub fn from_str(string: &str) -> CharString {
        CharString(string.chars().collect())
    }
}

impl fmt::Display for CharString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in &self.0 {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

/// Set of string tokens representing a complete phrase.
#[derive(Clone, Eq, PartialEq)]
pub struct Phrase(pub Vec<CharString>);
impl Phrase {
    /// Length of the strings joined on a single character
    pub fn joined_len(&self) -> usize {
        if self.0.is_empty() { return 0; }
        let char_total = self.0
            .iter()
            .fold(0, |total_len, char_str| total_len + char_str.0.len());
        let char_total_with_spaces = char_total + self.0.len() - 1;
        char_total_with_spaces
    }

    pub fn from_str(string: &str) -> Phrase {
        let phrase: Vec<_> = string
            .split(" ")
            .filter_map(|str|
                if str.is_empty() { None }
                else { Some(CharString::from_str(str)) }
            )
            .collect();
        Phrase(phrase)
    }
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, ch_str) in self.0.iter().enumerate() {
            write!(f, "{}", ch_str.to_string())?;
            if i != self.0.len() - 1 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

/// Instance of a phrase in a particular file or series of bytes.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PhraseInstance {
    /// Index of the phrase
    pub index: usize,
    /// Position in the file where the instance was found (byte offset).
    pub pos: usize,
    /// Context of the phrase instance (what was the surrounding text?)
    pub context: String
}

pub trait IntoPhrase {
    fn into_char_string(&self) -> Phrase {
        todo!()
    }
}

/// Bytes-per character flags (1, 2, 4)
pub type BpcFlags = u8;
const BPC_1: BpcFlags = 0b00000001;
const BPC_2: BpcFlags = 0b00000010;
const BPC_4: BpcFlags = 0b00000100;

/// Finds text in a buffered input stream
pub struct TextFinder<R: Read> {
    /// Source of bytes
    read: R,
    /// Phrases to search for
    phrases: Vec<Phrase>,
    /// Main "sliding" byte buffer
    buffer: SliceRingBuffer<u8>,
    /// Bytes-per-character flags
    bpc_flags: BpcFlags
}

impl<R: Read> TextFinder<R> {
    pub fn new(
        read: R,
        phrases: Vec<Phrase>,
        context_size: usize,
        bpc_flags: BpcFlags
    ) -> Self {
        for phrase in &phrases {
            if context_size < phrase.joined_len() {
                panic!("Phrase had ");
            }
        }
        Self {
            read,
            phrases,
            buffer: SliceRingBuffer::with_capacity(context_size*4),
            bpc_flags
        }
    }
}

impl<R: Read> Iterator for TextFinder<R> {
    type Item = PhraseInstance;
    fn next(&mut self) -> Option<Self::Item> {
        
        todo!()
    }
}

#[cfg(test)]
mod test {
    
    use crate::{Phrase, CharString};

    #[test]
    fn joined_len() {
        let phrase = Phrase::from_str("Testing 123 testing");
        assert_eq!(19, phrase.joined_len());

        let phrase = Phrase::from_str("Testing");
        assert_eq!(7, phrase.joined_len());

        let phrase = Phrase::from_str("");
        assert_eq!(0, phrase.joined_len());

        let phrase = Phrase::from_str("Testing    123    testing");
        assert_eq!(19, phrase.joined_len());
    }

    #[test]
    fn to_string_char_str() {
        let ch_str = CharString::from_str("Testing 123");
        assert_eq!(String::from("Testing 123"), ch_str.to_string());
    }

    #[test]
    fn to_string_phrase() {
        let phrase = Phrase::from_str("Testing 123");
        assert_eq!(String::from("Testing 123"), phrase.to_string());

        let phrase = Phrase::from_str("Testing     123   ");
        assert_eq!(String::from("Testing 123"), phrase.to_string());
    }
}