use super::AssignmentExpression;
use crate::syntax::{
    ast::node::Node,
    parser::{error::ParseResult, AllowAwait, AllowIn, Cursor, TokenParser},
};

use std::io::Read;

/// <https://tc39.es/ecma262/#prod-ExpressionBody>
#[derive(Debug, Clone, Copy)]
pub(super) struct ExpressionBody {
    allow_in: AllowIn,
    allow_await: AllowAwait,
}

impl ExpressionBody {
    /// Creates a new `ExpressionBody` parser.
    pub(super) fn new<I, A>(allow_in: I, allow_await: A) -> Self
    where
        I: Into<AllowIn>,
        A: Into<AllowAwait>,
    {
        Self {
            allow_in: allow_in.into(),
            allow_await: allow_await.into(),
        }
    }
}

impl<R> TokenParser<R> for ExpressionBody
where
    R: Read,
{
    type Output = Node;

    fn parse(self, cursor: &mut Cursor<R>) -> ParseResult {
        AssignmentExpression::new(self.allow_in, false, self.allow_await).parse(cursor)
    }
}
