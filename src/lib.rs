//! Big Bag Of Words
//!
//! The "Big Bag Of Words" is used in text analysis and
//! machine learning.  It reduces a text to a collection of
//! words, each with a count of the number of occurrences.
//!
//! This implementation uses zero-copy strings when
//! reasonably possible to improve performance and reduce
//! memory usage.
//!
//! Words are separated by whitespace, and consist of a
//! span of one or more consecutive letters (any Unicode
//! code point in the "letter" class) with no internal
//! punctuation: leading and trailing punctuation are
//! removed.
//!
//! For example, the text
//!
//! ```text
//! "It ain't over untïl it ain't, over."
//! ```
//!
//! contains the sequence of words `"It"`, `"over"`,
//! `"untïl"`, `"it"`, `"over"`.
//!
//! Words in the bag containing uppercase letters will be
//! represented by their lowercase equivalent.

use std::borrow::Cow;
use std::collections::BTreeMap;

/// Each key in this struct's map is a word in some
/// in-memory text document. The corresponding value is the
/// count of occurrences.
#[derive(Debug, Default, Clone)]
pub struct Bbow<'a>(BTreeMap<Cow<'a, str>, usize>);

fn is_word(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_alphabetic())
}

fn has_uppercase(word: &str) -> bool {
    word.chars().any(char::is_uppercase)
}

impl<'a> Bbow<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this BBOW.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up a BBOW covering
    /// multiple texts.
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        let string_parts = target.split_whitespace();
        let punctuation: &[_] = &["!", ".", ",", "?", "/", ";", ":", "'"];
        for parts in string_parts {
            let mut part = parts;
            // I know this is ugly, I tried to get that trim_matches function to work for like half an hour and got annoyed.
            for p in punctuation {
                part = part.trim_end_matches(p);
                part = part.trim_start_matches(p);
            }
            if is_word(part) {
                if has_uppercase(part) {
                    let lower = part.to_lowercase();
                    // This line of code was derived from the example at https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.entry
                    self.0
                        .entry(lower.into())
                        .and_modify(|curr| *curr += 1)
                        .or_insert(1);
                } else {
                    self.0
                        .entry(part.into())
                        .and_modify(|curr| *curr += 1)
                        .or_insert(1);
                }
            }
        }

        self
    }

    /// Report the number of occurrences of the given
    /// `keyword` that are indexed by this BBOW. The keyword
    /// should be lowercase and not contain punctuation, as
    /// per the rules of BBOW: otherwise the keyword will
    /// not match and 0 will be returned.
    pub fn match_count(&self, keyword: &str) -> usize {
        // let value = self.0.get(keyword);
        // let return_value: usize = Some(&value);
        // return_value
        self.0[keyword]
    }

    pub fn words(&'a self) -> impl Iterator<Item = &'a str> {
        self.0.keys().map(|w| w.as_ref())
    }

    /// Count the overall number of words contained in this BBOW:
    /// multiple occurrences are considered separate.
    ///
    pub fn count(&self) -> usize {
        let mut total = 0;
        for value in self.0.values() {
            println!("{}", value);
            total += value;
        }
        total
    }

    /// Count the number of unique words contained in this BBOW,
    /// not considering number of occurrences.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is this BBOW empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend_from_text() {
        let mut my_bag = Bbow::new();
        my_bag = my_bag.extend_from_text("This is a test string for test purposes");
        assert_eq!(7, my_bag.len());
    }

    #[test]
    fn test_adding_to_bag() {
        let mut my_bag = Bbow::new();
        my_bag = my_bag.extend_from_text("First three words");
        my_bag = my_bag.extend_from_text("Next. two? words,");
        assert_eq!(5, my_bag.len());
    }

    #[test]
    fn test_match_count() {
        let mut my_bag = Bbow::new();
        my_bag = my_bag.extend_from_text("b b b-banana b");
        assert_eq!(3, my_bag.match_count("b"));
    }

    #[test]
    fn test_len() {
        let mut my_bag = Bbow::new();
        my_bag = my_bag.extend_from_text("Can't stop this! Stop!");
        assert_eq!(2, my_bag.len());
    }

    #[test]
    fn test_count() {
        let mut bbow = Bbow::new();
        bbow = bbow.extend_from_text("Can't stop this! Stop!");
        assert_eq!(3, bbow.count());
    }

    #[test]
    fn test_is_empty(){
        let bbow = Bbow::new();
        assert_eq!(true, bbow.is_empty());
    }

    #[test]
    fn test_is_empty_with_words(){
        let mut bbow = Bbow::new();
        bbow = bbow.extend_from_text("Now there is something in here");
        assert_eq!(false, bbow.is_empty());
    }
}
