use tantivy::tokenizer::{BoxTokenStream, Tokenizer};

use crate::token_stream::TinySegmenterTokenStream;

/// Tokenizer for Japanese text, based on TinySegmenter.
#[derive(Clone)]
pub struct TinySegmenterTokenizer;

impl Tokenizer for TinySegmenterTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        TinySegmenterTokenStream::new(text).into()
    }
}
