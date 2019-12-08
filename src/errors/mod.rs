//! This module contains several error error types and their implementations

use std::error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    ExpectedIdentifier(error_kinds::ExpectedIdentifier),
    ExpectedToken(error_kinds::ExpectedToken),
    ExpectedEscape(error_kinds::ExpectedEscape),
    UnexpectedToken(error_kinds::UnexpectedToken),
    InvalidEscape(error_kinds::InvalidEscape),
    InvalidIdentifier(error_kinds::InvalidIdentifier),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ExpectedIdentifier(err) => write!(f, "{}", err),
            Error::ExpectedToken(err)      => write!(f, "{}", err),
            Error::ExpectedEscape(err)     => write!(f, "{}", err),
            Error::UnexpectedToken(err)    => write!(f, "{}", err),
            Error::InvalidEscape(err)      => write!(f, "{}", err),
            Error::InvalidIdentifier(err)  => write!(f, "{}", err),
        }
    }
}

/// Contains all the error types used in `Error`'s variants
pub mod error_kinds {
    use std::error;
    use std::fmt::{self, Display};

    #[derive(Debug)]
    pub struct ExpectedIdentifier {
        index: usize,
        line: String,
    }

    impl error::Error for ExpectedIdentifier {}

    impl Display for ExpectedIdentifier {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Expected identifier {}{{here}}{}", &self.line[..self.index], &self.line[self.index..])
        }
    }

    impl ExpectedIdentifier {
        /// Creates a new `ExpectedIdentifier` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured. Should be complete
        /// 
        /// `index`: the index where the identifier is expected
        /// 
        /// # Panics
        /// Panics if index is too big
        pub fn new(line: String, index: usize) -> ExpectedIdentifier {
            assert!(line.len() > index, "`index` must be a valid index in `line`");

            ExpectedIdentifier {
                line,
                index,
            }
        }
    }

    #[derive(Debug)]
    pub struct ExpectedToken {
        index: usize,
        line: String,
        tokens: String,
    }

    impl error::Error for ExpectedToken {}

    impl Display for ExpectedToken {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Expected {} {}{{here}}{}", self.tokens, &self.line[..self.index], &self.line[self.index..])
        }
    }

    impl ExpectedToken {
        /// Creates a new `ExpectedToken` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured. Should be complete
        /// 
        /// `index`: the index where the token is expected
        /// 
        /// `tokens`: the possible tokens. There is no rule to format it, you just should be aware this will be printed directly to the end user
        /// 
        /// # Panics
        /// Panics if `index` is too big
        pub fn new(line: String, index: usize, tokens: String) -> ExpectedToken {
            assert!(line.len() > index, "`index` must be a valid index");

            ExpectedToken {
                line,
                index,
                tokens,
            }
        }
    }

    #[derive(Debug)]
    pub struct ExpectedEscape {
        index: usize,
        line: String,
        replace: String,
        token: char,
    }

    impl error::Error for ExpectedEscape {}

    impl Display for ExpectedEscape {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Expected escape sequence {} instead of {} in {}{{here}}{}", 
                       self.replace,
                       self.token,
                       &self.line[..self.index],
                       &self.line[self.index + self.token.len_utf8()..])
        }
    }

    impl ExpectedEscape {
        /// Creates a new `ExpectedEscape` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured
        /// 
        /// `index`: the index of the error
        /// 
        /// `replace`: the escape sequence which should be used instead
        /// 
        /// # Panics
        /// Panics if `index` is too big or is at an invalid position
        pub fn new(line: String, index: usize, replace: String) -> ExpectedEscape {
            ExpectedEscape {
                token: super::nth_char(&line, index),
                line,
                replace,
                index,
            }
        }
    }

    #[derive(Debug)]
    pub struct UnexpectedToken {
        index: usize,
        line: String,
        token: char,
    }

    impl error::Error for UnexpectedToken {}

    impl Display for UnexpectedToken {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Unexpected token {} {}{{here}}",
                       self.token,
                       &self.line[..self.index])
        }
    }

    impl UnexpectedToken {
        /// Creates a new `UnexpectedToken` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured
        /// 
        /// `index`: the index where a token was not expected
        /// 
        /// # Panics
        /// Panics if `index` is too big or is at an invalid position
        pub fn new(line: String, index: usize) -> UnexpectedToken {
            UnexpectedToken {
                index,
                token: super::nth_char(&line, index),
                line,
            }
        }
    }

    #[derive(Debug)]
    pub struct InvalidEscape {
        line: String,
        escape: String,
    }

    impl error::Error for InvalidEscape {}

    impl Display for InvalidEscape {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid escape sequence {} in {}", self.escape, self.line)
        }
    }

    impl InvalidEscape {
        /// Creates a new `InvalidEscape` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured
        /// 
        /// `escape`: the escape sequence which is invalid
        /// 
        /// # Panics
        /// Panics if `escape` is not in `line`
        pub fn new(line: String, escape: String) -> InvalidEscape {
            assert!(line.find(&escape).is_some(), "`line` must contain `escape`");

            InvalidEscape {
                line,
                escape,
            }
        }
    }

    #[derive(Debug)]
    pub struct InvalidIdentifier {
        line: String,
        ident: String,
    }

    impl error::Error for InvalidIdentifier {}

    impl Display for InvalidIdentifier {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid identifier {} in {}", self.ident, self.line)
        }
    }

    impl InvalidIdentifier {
        /// Creates a new `InvalidIdentifier` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured
        /// 
        /// `identifier`: the identifier found. It must be invalid
        /// 
        /// # Panics
        /// Panics
        /// - if `identifier` is valid
        /// - if `identifier` is not in `line`
        pub fn new(line: String, identifier: String) -> InvalidIdentifier {
            assert!(line.find(&identifier).is_some(), "`line` must contain `identifier`");
            assert!(!crate::datas::Identifier::is_valid(&identifier), "`identifier` must be an invalid identifier");

            InvalidIdentifier {
                line,
                ident: identifier,
            }
        }
    }
}

/// Returns the character at the `index`th index (`index` is in bytes) in `string`
/// 
/// # Panics
/// Panics if `index` is out of range or between two bytes of the same character
fn nth_char(string: &str, index: usize) -> char {
    assert!(string.len() > index, "`index` must be a valid index in `string`");

    let mut token = '\0';
    let mut last_n = 0;

    for (n, i) in string.char_indices() {
        last_n = n;

        if n == index {
            token = i;
            break;
        } else if n > index {
            panic!("`index` is not a valid index in `string` (`index` = {:?}, `string` = {:?})", index, string);
        }
    }

    assert_eq!(last_n, index, "`index` is not a valid index in `string` (`index` = {:?}, `string` = {:?})", index, string);

    token
}


#[cfg(test)]
mod tests;
