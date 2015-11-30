use std::error::Error;
use std::fmt;
use compiler::parse::tokens::Close;
use compiler::parse::Span;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char, Span, String),
    UnterminatedString(Span),
    ConversionError(String, Box<Error>),
    BadEscape(Span, String),
    MissingRightDelimiter(Close),
    ExtraRightDelimiter(Close, Span),
    InvalidMapLiteral(Span),
    UnexpectedIfArity(usize, Span),
    UnexpectedLambdaArity(usize, Span),
    BadLambdaArgs(Span),
}

use self::ParseError::*;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnexpectedChar(c, span, ref while_doing) =>
                write!(f, "Unexpected character {} at {}, {}", c, span.start, while_doing),
            UnterminatedString(span) => write!(f, "Unterminated string beginning at {}", span.start),
            ConversionError(ref s, ref e) => {
                write!(f, "Could not convert {}: {}", s, e)
            }
            BadEscape(span, ref s) =>
                write!(f, "Invalid escape sequence starting at {}: {}", span.start, s),
            MissingRightDelimiter(c) => write!(f, "Missing right delimiter {}", c.to_char()),
            ExtraRightDelimiter(c, span) =>
                write!(f, "Extra right delimiter {} at {}", c.to_char(), span.start),
            InvalidMapLiteral(span) => write!(f, "Map literal at {} is malformed", span.start),
            UnexpectedIfArity(size, span) =>
                write!(f, "`if` at {} takes {} arguments.  It should take 3", span.start, size),
            UnexpectedLambdaArity(_, span) =>
                write!(f, "`lambda` at {} takes at least an args list and one body.", span.start),
            BadLambdaArgs(span) =>
                write!(f, "Malformed arguments list at {}", span.start),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            UnexpectedChar(_, _, _) => "Unexpected character",
            UnterminatedString(_) => "Unterminated string",
            ConversionError(_, ref e) => e.description(),
            BadEscape(..) => "Bad escape sequence",
            MissingRightDelimiter(..) => "Missing right delimiter",
            ExtraRightDelimiter(..) => "Extra right delimiter",
            InvalidMapLiteral(..) => "Map literals require an even number of elements",
            UnexpectedIfArity(..) => "Wrong arity for \"if\" expression",
            UnexpectedLambdaArity(..) => "Wrong arity for \"lambda\" expression",
            BadLambdaArgs(..) => "arguments to \"lambda\" are malformed",
        }
    }
}
