use std::iter::Enumerate;
use tantivy::tokenizer::{Token, TokenStream};

pub struct TinySegmenterTokenStream {
    tinyseg_enum: Enumerate<std::vec::IntoIter<String>>,
    current_token: Token,
    offset_from: usize,
    offset_to: usize,
}

impl TinySegmenterTokenStream {
    pub fn new(text: &str) -> TinySegmenterTokenStream {
        TinySegmenterTokenStream {
            tinyseg_enum: tinysegmenter::tokenize(text).into_iter().enumerate(),
            current_token: Token::default(),
            offset_from: 0,
            offset_to: 0,
        }
    }
}

impl TokenStream for TinySegmenterTokenStream {
    fn advance(&mut self) -> bool {
        match self.tinyseg_enum.next() {
            Some((pos, term)) => {
                self.offset_from = self.offset_to;
                self.offset_to = self.offset_from + term.len();

                let offset_from = self.offset_from;
                let offset_to = self.offset_to;

                self.current_token = Token {
                    offset_from,
                    offset_to,
                    position: pos,
                    text: term,
                    position_length: 1,
                };

                return true;
            }

            None => return false,
        }
    }

    fn token(&self) -> &Token {
        &self.current_token
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.current_token
    }
}
