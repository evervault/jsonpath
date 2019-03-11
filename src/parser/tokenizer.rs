use std::result;
use std::io::Write;

use super::path_reader::{
    ReaderError,
    PathReader,
};

const ABSOLUTE: &'static str = "$";
const DOT: &'static str = ".";
const AT: &'static str = "@";
const OPEN_ARRAY: &'static str = "[";
const CLOSE_ARRAY: &'static str = "]";
const ASTERISK: &'static str = "*";
const QUESTION: &'static str = "?";
const COMMA: &'static str = ",";
const SPLIT: &'static str = ":";
const OPEN_PARENTHESIS: &'static str = "(";
const CLOSE_PARENTHESIS: &'static str = ")";
const KEY: &'static str = "Key";
const DOUBLE_QUOTA: &'static str = "\"";
const SINGLE_QUOTA: &'static str = "'";
const EQUAL: &'static str = "==";
const GREATER_OR_EQUAL: &'static str = ">=";
const GREATER: &'static str = ">";
const LITTLE: &'static str = "<";
const LITTLE_OR_EQUAL: &'static str = "<=";
const NOT_EQUAL: &'static str = "!=";
const AND: &'static str = "&&";
const OR: &'static str = "||";
const WHITESPACE: &'static str = " ";

const CH_DOLLA: char = '$';
const CH_DOT: char = '.';
const CH_ASTERISK: char = '*';
const CH_LARRAY: char = '[';
const CH_RARRAY: char = ']';
const CH_LPAREN: char = '(';
const CH_RPAREN: char = ')';
const CH_AT: char = '@';
const CH_QUESTION: char = '?';
const CH_COMMA: char = ',';
const CH_SEMICOLON: char = ':';
const CH_EQUAL: char = '=';
const CH_AMPERSAND: char = '&';
const CH_PIPE: char = '|';
const CH_LITTLE: char = '<';
const CH_GREATER: char = '>';
const CH_EXCLAMATION: char = '!';
const CH_SINGLE_QUOTA: char = '\'';
const CH_DOUBLE_QUOTA: char = '"';

#[derive(Debug, Clone, PartialEq)]
pub enum TokenError {
    Eof,
    Position(usize),
}

fn to_token_error(read_err: ReaderError) -> TokenError {
    match read_err {
        ReaderError::Eof => TokenError::Eof
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Absolute(usize),
    Dot(usize),
    At(usize),
    OpenArray(usize),
    CloseArray(usize),
    Asterisk(usize),
    Question(usize),
    Comma(usize),
    Split(usize),
    OpenParenthesis(usize),
    CloseParenthesis(usize),
    Key(usize, String),
    DoubleQuoted(usize, String),
    SingleQuoted(usize, String),
    Equal(usize),
    GreaterOrEqual(usize),
    Greater(usize),
    Little(usize),
    LittleOrEqual(usize),
    NotEqual(usize),
    And(usize),
    Or(usize),
    Whitespace(usize, usize),
}

impl Token {

    pub fn partial_eq(&self, other: Token) -> bool {
        self.to_simple() == other.to_simple()
    }

    fn to_simple(&self) -> &'static str {
        match self {
            Token::Absolute(_) => ABSOLUTE,
            Token::Dot(_) => DOT,
            Token::At(_) => AT,
            Token::OpenArray(_) => OPEN_ARRAY,
            Token::CloseArray(_) => CLOSE_ARRAY,
            Token::Asterisk(_) => ASTERISK,
            Token::Question(_) => QUESTION,
            Token::Comma(_) => COMMA,
            Token::Split(_) => SPLIT,
            Token::OpenParenthesis(_) => OPEN_PARENTHESIS,
            Token::CloseParenthesis(_) => CLOSE_PARENTHESIS,
            Token::Key(_, _) => KEY,
            Token::DoubleQuoted(_, _) => DOUBLE_QUOTA,
            Token::SingleQuoted(_, _) => SINGLE_QUOTA,
            Token::Equal(_) => EQUAL,
            Token::GreaterOrEqual(_) => GREATER_OR_EQUAL,
            Token::Greater(_) => GREATER,
            Token::Little(_) => LITTLE,
            Token::LittleOrEqual(_) => LITTLE_OR_EQUAL,
            Token::NotEqual(_) => NOT_EQUAL,
            Token::And(_) => AND,
            Token::Or(_) => OR,
            Token::Whitespace(_, _) => WHITESPACE
        }
    }
}

fn simple_matched_token(ch: char, pos: usize) -> Option<Token> {
    match ch {
        CH_DOLLA => Some(Token::Absolute(pos)),
        CH_DOT => Some(Token::Dot(pos)),
        CH_ASTERISK => Some(Token::Asterisk(pos)),
        CH_LARRAY => Some(Token::OpenArray(pos)),
        CH_RARRAY => Some(Token::CloseArray(pos)),
        CH_LPAREN => Some(Token::OpenParenthesis(pos)),
        CH_RPAREN => Some(Token::CloseParenthesis(pos)),
        CH_AT => Some(Token::At(pos)),
        CH_QUESTION => Some(Token::Question(pos)),
        CH_COMMA => Some(Token::Comma(pos)),
        CH_SEMICOLON => Some(Token::Split(pos)),
        _ => None
    }
}

pub struct Tokenizer<'a> {
    input: PathReader<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input: PathReader::new(input),
        }
    }

    fn single_quota(&mut self, pos: usize, ch: char) -> result::Result<Token, TokenError> {
        let (_, val) = self.input.take_while(|c| *c != ch).map_err(to_token_error)?;
        self.input.next_char().map_err(to_token_error)?;
        Ok(Token::SingleQuoted(pos, val))
    }

    fn double_quota(&mut self, pos: usize, ch: char) -> result::Result<Token, TokenError> {
        let (_, val) = self.input.take_while(|c| *c != ch).map_err(to_token_error)?;
        self.input.next_char().map_err(to_token_error)?;
        Ok(Token::DoubleQuoted(pos, val))
    }

    fn equal(&mut self, pos: usize, _: char) -> result::Result<Token, TokenError> {
        let (_, ch) = self.input.peek_char().map_err(to_token_error)?;
        match ch {
            CH_EQUAL => {
                self.input.next_char().map_err(to_token_error)?;
                Ok(Token::Equal(pos))
            }
            _ => Err(TokenError::Position(pos))
        }
    }

    fn not_equal(&mut self, pos: usize, _: char) -> result::Result<Token, TokenError> {
        let (_, ch) = self.input.peek_char().map_err(to_token_error)?;
        match ch {
            CH_EQUAL => {
                self.input.next_char().map_err(to_token_error)?;
                Ok(Token::NotEqual(pos))
            }
            _ => Err(TokenError::Position(pos))
        }
    }

    fn little(&mut self, pos: usize, _: char) -> result::Result<Token, TokenError> {
        let (_, ch) = self.input.peek_char().map_err(to_token_error)?;
        match ch {
            CH_EQUAL => {
                self.input.next_char().map_err(to_token_error)?;
                Ok(Token::LittleOrEqual(pos))
            }
            _ => Ok(Token::Little(pos)),
        }
    }

    fn greater(&mut self, pos: usize, _: char) -> result::Result<Token, TokenError> {
        let (_, ch) = self.input.peek_char().map_err(to_token_error)?;
        match ch {
            CH_EQUAL => {
                self.input.next_char().map_err(to_token_error)?;
                Ok(Token::GreaterOrEqual(pos))
            }
            _ => Ok(Token::Greater(pos)),
        }
    }

    fn and(&mut self, pos: usize, _: char) -> result::Result<Token, TokenError> {
        let (_, ch) = self.input.peek_char().map_err(to_token_error)?;
        match ch {
            CH_AMPERSAND => {
                let _ = self.input.next_char().map_err(to_token_error);
                Ok(Token::And(pos))
            }
            _ => Err(TokenError::Position(pos))
        }
    }

    fn or(&mut self, pos: usize, _: char) -> result::Result<Token, TokenError> {
        let (_, ch) = self.input.peek_char().map_err(to_token_error)?;
        match ch {
            CH_PIPE => {
                self.input.next_char().map_err(to_token_error)?;
                Ok(Token::Or(pos))
            }
            _ => Err(TokenError::Position(pos))
        }
    }

    fn whitespace(&mut self, pos: usize, _: char) -> result::Result<Token, TokenError> {
        let (_, vec) = self.input.take_while(|c| c.is_whitespace()).map_err(to_token_error)?;
        Ok(Token::Whitespace(pos, vec.len()))
    }

    fn other(&mut self, pos: usize, ch: char) -> result::Result<Token, TokenError> {
        let fun = |c: &char| {
            match simple_matched_token(*c, pos) {
                Some(_) => false,
                _ if c == &CH_LITTLE
                    || c == &CH_GREATER
                    || c == &CH_EQUAL
                    || c == &CH_AMPERSAND
                    || c == &CH_PIPE
                    || c == &CH_EXCLAMATION => false,
                _ => !c.is_whitespace()
            }
        };
        let (_, mut vec) = self.input.take_while(fun).map_err(to_token_error)?;
        vec.insert(0, ch);
        Ok(Token::Key(pos, vec))
    }

    pub fn next_token(&mut self) -> result::Result<Token, TokenError> {
        let (pos, ch) = self.input.next_char().map_err(to_token_error)?;
        match simple_matched_token(ch, pos) {
            Some(t) => Ok(t),
            None => {
                match ch {
                    CH_SINGLE_QUOTA => self.single_quota(pos, ch),
                    CH_DOUBLE_QUOTA => self.double_quota(pos, ch),
                    CH_EQUAL => self.equal(pos, ch),
                    CH_GREATER => self.greater(pos, ch),
                    CH_LITTLE => self.little(pos, ch),
                    CH_AMPERSAND => self.and(pos, ch),
                    CH_PIPE => self.or(pos, ch),
                    CH_EXCLAMATION => self.not_equal(pos, ch),
                    _ if ch.is_whitespace() => self.whitespace(pos, ch),
                    _ => self.other(pos, ch),
                }
            }
        }
    }

    fn current_pos(&self) -> usize {
        self.input.current_pos()
    }
}

pub struct PreloadedTokenizer<'a> {
    origin_input: &'a str,
    err: TokenError,
    err_pos: usize,
    tokens: Vec<(usize, Token)>,
    curr_pos: Option<usize>,
}

impl<'a> PreloadedTokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = vec![];
        loop {
            match tokenizer.next_token() {
                Ok(t) => {
                    tokens.insert(0, (tokenizer.current_pos(), t));
                }
                Err(e) => {
                    return PreloadedTokenizer {
                        origin_input: input.clone(),
                        err: e,
                        err_pos: tokenizer.current_pos(),
                        tokens,
                        curr_pos: None,
                    };
                }
            }
        }
    }

    pub fn peek_token(&self) -> result::Result<&Token, TokenError> {
        match self.tokens.last() {
            Some((_, t)) => {
                trace!("%{:?}", t);
                Ok(t)
            }
            _ => {
                trace!("%{:?}", self.err);
                Err(self.err.clone())
            }
        }
    }

    pub fn next_token(&mut self) -> result::Result<Token, TokenError> {
        match self.tokens.pop() {
            Some((pos, t)) => {
                self.curr_pos = Some(pos);
                trace!("@{:?}", t);
                Ok(t)
            }
            _ => {
                trace!("@{:?}", self.err);
                Err(self.err.clone())
            }
        }
    }

    pub fn err_msg_with_pos(&self, pos: usize) -> String {
        let mut w = Vec::new();
        writeln!(&mut w, "{}", self.origin_input).unwrap();
        writeln!(&mut w, "{}", "^".repeat(pos)).unwrap();
        match std::str::from_utf8(&w[..]) {
            Ok(s) => s.to_owned(),
            Err(_) => panic!("Invalid UTF-8")
        }
    }

    pub fn err_msg(&self) -> String {
        match self.curr_pos {
            Some(pos) => {
                self.err_msg_with_pos(pos)
            }
            _ => {
                self.err_msg_with_pos(self.err_pos)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TokenError;
    use super::{
        Token,
        Tokenizer,
        PreloadedTokenizer,
    };

    fn collect_token(input: &str) -> (Vec<Token>, Option<TokenError>) {
        let mut tokenizer = Tokenizer::new(input);
        let mut vec = vec![];
        loop {
            match tokenizer.next_token() {
                Ok(t) => vec.push(t),
                Err(e) => return (vec, Some(e)),
            }
        }
    }

    fn run(input: &str, expected: (Vec<Token>, Option<TokenError>)) {
        let (vec, err) = collect_token(input.clone());
        assert_eq!((vec, err), expected, "\"{}\"", input);
    }

    #[test]
    fn peek() {
        let mut tokenizer = PreloadedTokenizer::new("$.a");
        match tokenizer.next_token() {
            Ok(t) => assert_eq!(Token::Absolute(0), t),
            _ => panic!()
        }

        match tokenizer.peek_token() {
            Ok(t) => assert_eq!(&Token::Dot(1), t),
            _ => panic!()
        }

        match tokenizer.peek_token() {
            Ok(t) => assert_eq!(&Token::Dot(1), t),
            _ => panic!()
        }

        match tokenizer.next_token() {
            Ok(t) => assert_eq!(Token::Dot(1), t),
            _ => panic!()
        }
    }

    #[test]
    fn token() {
        run("$.01.a",
            (
                vec![
                    Token::Absolute(0),
                    Token::Dot(1),
                    Token::Key(2, "01".to_string()),
                    Token::Dot(4),
                    Token::Key(5, "a".to_string())
                ]
                , Some(TokenError::Eof)
            ));

        run("$.   []",
            (
                vec![
                    Token::Absolute(0),
                    Token::Dot(1),
                    Token::Whitespace(2, 2),
                    Token::OpenArray(5),
                    Token::CloseArray(6)
                ]
                , Some(TokenError::Eof)
            ));

        run("$..",
            (
                vec![
                    Token::Absolute(0),
                    Token::Dot(1),
                    Token::Dot(2),
                ]
                , Some(TokenError::Eof)
            ));

        run("$..ab",
            (
                vec![
                    Token::Absolute(0),
                    Token::Dot(1),
                    Token::Dot(2),
                    Token::Key(3, "ab".to_string())
                ]
                , Some(TokenError::Eof)
            ));

        run("$..가 [",
            (
                vec![
                    Token::Absolute(0),
                    Token::Dot(1),
                    Token::Dot(2),
                    Token::Key(3, "가".to_string()),
                    Token::Whitespace(6, 0),
                    Token::OpenArray(7),
                ]
                , Some(TokenError::Eof)
            ));

        run("[-1, 2 ]",
            (
                vec![
                    Token::OpenArray(0),
                    Token::Key(1, "-1".to_string()),
                    Token::Comma(3),
                    Token::Whitespace(4, 0),
                    Token::Key(5, "2".to_string()),
                    Token::Whitespace(6, 0),
                    Token::CloseArray(7),
                ]
                , Some(TokenError::Eof)
            ));

        run("[ 1 2 , 3 \"abc\" : -10 ]",
            (
                vec![
                    Token::OpenArray(0),
                    Token::Whitespace(1, 0),
                    Token::Key(2, "1".to_string()),
                    Token::Whitespace(3, 0),
                    Token::Key(4, "2".to_string()),
                    Token::Whitespace(5, 0),
                    Token::Comma(6),
                    Token::Whitespace(7, 0),
                    Token::Key(8, "3".to_string()),
                    Token::Whitespace(9, 0),
                    Token::DoubleQuoted(10, "abc".to_string()),
                    Token::Whitespace(15, 0),
                    Token::Split(16),
                    Token::Whitespace(17, 0),
                    Token::Key(18, "-10".to_string()),
                    Token::Whitespace(21, 0),
                    Token::CloseArray(22),
                ]
                , Some(TokenError::Eof)
            ));

        run("?(@.a가 <41.01)",
            (
                vec![
                    Token::Question(0),
                    Token::OpenParenthesis(1),
                    Token::At(2),
                    Token::Dot(3),
                    Token::Key(4, "a가".to_string()),
                    Token::Whitespace(8, 0),
                    Token::Little(9),
                    Token::Key(10, "41".to_string()),
                    Token::Dot(12),
                    Token::Key(13, "01".to_string()),
                    Token::CloseParenthesis(15),
                ]
                , Some(TokenError::Eof)
            ));

        run("?(@.a <4a.01)",
            (
                vec![
                    Token::Question(0),
                    Token::OpenParenthesis(1),
                    Token::At(2),
                    Token::Dot(3),
                    Token::Key(4, "a".to_string()),
                    Token::Whitespace(5, 0),
                    Token::Little(6),
                    Token::Key(7, "4a".to_string()),
                    Token::Dot(9),
                    Token::Key(10, "01".to_string()),
                    Token::CloseParenthesis(12),
                ]
                , Some(TokenError::Eof)
            ));

        run("?($.c>@.d)", (
            vec![
                Token::Question(0),
                Token::OpenParenthesis(1),
                Token::Absolute(2),
                Token::Dot(3),
                Token::Key(4, "c".to_string()),
                Token::Greater(5),
                Token::At(6),
                Token::Dot(7),
                Token::Key(8, "d".to_string()),
                Token::CloseParenthesis(9)
            ]
            , Some(TokenError::Eof)
        ));
    }
}