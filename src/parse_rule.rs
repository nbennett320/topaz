use crate::precedence::Precedence;
use crate::Parser;

pub struct ParseRule {
    pub prefix: Option<fn(parser: &mut Parser)>,
    pub infix: Option<fn(parser: &mut Parser)>,
    pub precedence: Precedence,
}
