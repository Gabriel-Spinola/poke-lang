use std::io::Read;

use macros::AllVariants;

use super::{ParseFn, Parser};

#[repr(u8)]
#[derive(Debug, AllVariants)]
pub enum TokenRule {
    // Keywords
    And,
    Do,
    Then,
    If,
    Else,
    ElseIf,
    End,
    For,
    In,
    Function,
    Mut,
    Nil,
    Not,
    Or,
    While,
    Repeat,
    Return,
    Until,
    Require,
    Break,

    // Operations
    //   +     -   *    /    %    ^    #
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Len,

    //    &       ~       |       <<      >>     //
    BitAnd,
    BitOr,
    BitNot,
    ShiftL,
    ShiftR,
    Idiv,

    //   ==       ~=     <=      >=      <       >     =
    Equal,
    NotEq,
    LesEq,
    GreEq,
    Less,
    Greater,
    Assign,

    //  (       )       {      }      [       ]      ::
    ParL,
    ParR,
    CurlyL,
    CurlyR,
    SqurL,
    SqurR,
    DoubColon,

    //      ;        :       ,      .    <>     ..     ->
    SemiColon,
    Colon,
    Comma,
    Dot,
    Concat,
    Dots,
    Arrow,

    // Data types (refers to to their actual value no keywords)
    Int,
    Float,
    String,
    Bool,
    Byte,

    Identifier,

    // End of line
    EoS,
}

pub const RULES_COUNT: usize = (TokenRule::EoS as usize) + 1;

pub enum Precedence {
    None,
    Assignment, // = ->
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // - ~ not #
    Call,       // : . ()
    Primary,
}

impl Default for Precedence {
    fn default() -> Self {
        Self::None
    }
}

pub struct ParseRule<'a, R: Read> {
    pub prefix: Option<ParseFn<'a, R>>,
    pub infix: Option<ParseFn<'a, R>>,

    pub precedence: Precedence,
}

impl<'a, R: Read> Default for ParseRule<'a, R> {
    fn default() -> Self {
        Self {
            prefix: Default::default(),
            infix: Default::default(),
            precedence: Default::default(),
        }
    }
}

// REVIEW - I hate this
// REVIEW - Maybe infix only tokens don't need a Token Rule, because they can be directly converted to bytecode (numeric type for example)
impl<'a, R: Read> ParseRule<'a, R> {
    pub fn get_rule(token: TokenRule) -> &'a ParseRule<'a, R> {
        &Self::rules()[token as usize]
    }

    pub fn rules() -> &'a [ParseRule<'a, R>; RULES_COUNT] {
        &[
            // And
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Do
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Then,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // If,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // ElseIf,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Else,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // End,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // For,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // In,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Function,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Mut,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Nil,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Not,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Or,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // While,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Repeat,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Return,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Until,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Require,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Break,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Add,
            ParseRule {
                prefix: None,
                infix: Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_binary_op(current_token.unwrap())
                }),
                precedence: Precedence::Term,
            },
            // Sub,
            ParseRule {
                prefix: Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_unary_op(current_token.unwrap())
                }),
                infix: Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_binary_op(current_token.unwrap())
                }),
                precedence: Precedence::Term,
            },
            // Mul,
            ParseRule {
                prefix: None,
                infix: Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_binary_op(current_token.unwrap())
                }),
                precedence: Precedence::Factor,
            },
            // Div,
            ParseRule {
                prefix: None,
                infix: Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_binary_op(current_token.unwrap())
                }),
                precedence: Precedence::Factor,
            },
            // Mod,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Pow,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Len,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // BitAnd,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // BitOr,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // BitNot,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // ShiftL,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // ShiftR,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Idiv,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Equal,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // NotEq,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // LesEq,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // GreEq,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Less,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Greater,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Assign,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // ParL,
            ParseRule {
                prefix: Some(|parser: &mut Parser<'_, R>, _| parser.parse_grouping()),
                infix: None,
                precedence: Precedence::None,
            },
            // ParR,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // CurlyL,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // CurlyR,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // SqurL,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // SqurR,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // DoubColon,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // SemiColon,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Colon,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Comma,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Dot,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Concat,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Dots,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Arrow,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Int,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Float,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // String,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Bool,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Byte,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // Identifier,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // EoS,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        ]
    }
}
