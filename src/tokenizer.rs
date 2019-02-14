use tantivy::tokenizer::Tokenizer;

use crate::token_stream::TinySegmenterTokenStream;

/// Tokenizer for Japanese text, based on TinySegmenter.
#[derive(Clone)]
pub struct TinySegmenterTokenizer;

impl<'a> Tokenizer<'a> for TinySegmenterTokenizer {
    type TokenStreamImpl = TinySegmenterTokenStream;

    fn token_stream(&self, text: &'a str) -> Self::TokenStreamImpl {
        TinySegmenterTokenStream::new(text)
    }
}
