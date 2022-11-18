use std::io::Read;

use slice_ring_buffer::SliceRingBuffer;

/// String type where each character is stored as 4 bytes in a Vec.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CharString(Vec<char>);
impl CharString {
    pub fn from_str(string: &str) -> CharString {
        CharString(string.chars().collect())
    }
}

/// Set of string tokens representing a complete phrase.
#[derive(Clone, Eq, PartialEq, Debug)]
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

/// Any type that can be converted into a [`CharString`]
pub trait IntoCharString {
    fn into_char_string(&self) -> CharString;
}

impl IntoCharString for String {
    fn into_char_string(&self) -> CharString {
        CharString(self.chars().collect())
    }
}

pub trait IntoPhrase {
    fn into_char_string(&self) -> Phrase {
        todo!()
    }
}

/// Finds text in a buffered input stream
pub struct TextFinder<R: Read> {
    /// Source of bytes
    read: R,
    /// Phrases to search for
    phrases: Vec<Phrase>,
    /// Buffer that stores 1 bpc text
    context_1: SliceRingBuffer<u8>,
    /// Buffer that stores 2 bpc text
    context_2: SliceRingBuffer<u8>,
    /// Buffer that stores 4 bpc text
    context_4: SliceRingBuffer<u8>,
    /// Maximum bytes per character
    max_bpc: usize
}

impl<R: Read> TextFinder<R> {

    pub fn new(read: R, phrases: Vec<Phrase>, context_size: usize) -> Self {
        // for phrase in &phrases {
        //     if phrase.len() > 
        // }
        Self {
            read,
            phrases,
            context_1: SliceRingBuffer::with_capacity(context_size),
            context_2: SliceRingBuffer::with_capacity(context_size*2),
            context_4: SliceRingBuffer::with_capacity(context_size*4),
            max_bpc: 1
        }
    }

    pub fn with_max_bbc(mut self, max_bpc: usize) -> Self {
        if max_bpc == 0  { panic!("max_bpc cannot be 0") }
        if max_bpc > 4 { panic!("max_bpc cannot be greater than 4") }
        self.max_bpc = max_bpc;
        self
    }
}

#[cfg(test)]
mod test {
    
    use crate::Phrase;

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
}