use super::ExpressionBody;
use crate::syntax::{
    ast::{
        node::{Return, StatementList},
        Punctuator,
    },
    lexer::TokenKind,
    parser::{error::ParseError, function::FunctionBody, AllowIn, Cursor, TokenParser},
};

use std::io::Read;

/// ConciseBody / AsyncConciseBody parsing.
///
/// As these are effectively the same we just use a flag to reduce code duplication.
///
/// More information:
///  - [MDN documentation][mdn]
///  - [ECMAScript specification][spec]
///
/// Async More information:
///  - [MDN documentation][async-mdn]
///  - [ECMAScript specification][async-spec]
///
/// [mdn]:
/// [spec]: https://tc39.es/ecma262/#prod-ConciseBody
/// [mdn-async]:
/// [spec-async]: https://tc39.es/ecma262/#prod-AsyncConciseBody
#[derive(Debug, Clone, Copy)]
pub(super) struct ConciseBody {
    allow_in: AllowIn,
    is_async: bool,
}

impl ConciseBody {
    /// Creates a new `ConcideBody` parser.
    pub(super) fn new<I>(allow_in: I, is_async: bool) -> Self
    where
        I: Into<AllowIn>,
    {
        Self {
            allow_in: allow_in.into(),
            is_async,
        }
    }
}

impl<R> TokenParser<R> for ConciseBody
where
    R: Read,
{
    type Output = StatementList;

    fn parse(self, cursor: &mut Cursor<R>) -> Result<Self::Output, ParseError> {
        match cursor.peek(0)?.ok_or(ParseError::AbruptEnd)?.kind() {
            TokenKind::Punctuator(Punctuator::OpenBlock) => {
                let _ = cursor.next();
                let body = FunctionBody::new(false, self.is_async).parse(cursor)?;
                if self.is_async {
                    cursor.expect(Punctuator::CloseBlock, "async concise body")?;
                } else {
                    cursor.expect(Punctuator::CloseBlock, "concise body")?;
                }

                Ok(body)
            }
            _ => Ok(StatementList::from(vec![Return::new(
                ExpressionBody::new(self.allow_in, self.is_async).parse(cursor)?,
                None,
            )
            .into()])),
        }
    }
}
