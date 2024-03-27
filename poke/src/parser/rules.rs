use super::{ParseFn, Parser};
use crate::parser::tokens::TokenRule;
use std::io::Read;

/// Short for:
/// ```rust
/// ParseRule {
///     prefix: $firstParam
///     infix: $secondParam
///     precendence: $thirdParam
/// }
/// ```
macro_rules! parse_rule {
    ($prefix:expr, $infix:expr, $precedence:expr) => {
        ParseRule {
            prefix: $prefix,
            infix: $infix,
            precedence: $precedence,
        }
    };
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

/// Get the corresponding ParseRule for a given `TokenRule` enumerator
/// You can use normal Tokens by calling the to_rules macro
/// ```rust
/// assert_eq!(
///     ParseRule::<'_, R>::get_rule(Token::And.to_rule().unwrap()),
///     &Self::rules()[0]
/// )
/// ```
pub fn get_rule<'a, R: Read>(token: TokenRule) -> &'a ParseRule<'a, R> {
    &ParseRule::rules()[token as usize]
}

impl<'a, R: Read> ParseRule<'a, R> {
    /// REVIEW - I hate this
    /// REVIEW - Maybe infix-only tokens don't need to be in the token rules array because they can be directly converted to bytecode (numeric type, for example).
    ///
    /// ! The order of the rules should be the same as their respective TokenRule enumerators byte value
    ///
    /// NOTE - Why this is what it is:
    /// In the book, to create the parsing rules, The C99's designated initializer
    /// syntax is used. This feature is impossible in safe Rust. Also, in Rust, to
    /// use Tokens as an index, we would need to specify a data structure like
    /// vectors or hashmaps. However, considering this is a performance-critical
    /// part of the application, I don't think it's a good idea to store any of this
    /// data on the heap. So, in order to keep all of these operations as
    /// memory-efficient as possible (which is not much, considering I don't know Rust),
    /// I'm using a reference to a static array of parsing rules. Since it's static,
    /// Rust doesn't allow me to use the Default trait or late initialization by
    /// using TokenRules byte values. Also, implying that we're using references
    /// instead of copying everything is filled with lifetimes.
    ///
    /// Another little problem I encountered is that the whole existence of the
    /// `TokenRules` enum is because Rust doesn't allow me to just have a byte value
    /// assigned to an enumerator position if any of these enumerators store a typed
    /// value. I do think this solution here is memory-efficient and it seems
    /// to work, but I really damn hate the overall verbosity and lack of readability
    /// it turned out to have.
    pub fn rules() -> &'a [ParseRule<'a, R>; RULES_COUNT] {
        &[
            // And
            parse_rule!(None, None, Precedence::None),
            // Do
            parse_rule!(None, None, Precedence::None),
            // Then,
            parse_rule!(None, None, Precedence::None),
            // If,
            parse_rule!(None, None, Precedence::None),
            // ElseIf,
            parse_rule!(None, None, Precedence::None),
            // Else,
            parse_rule!(None, None, Precedence::None),
            // End,
            parse_rule!(None, None, Precedence::None),
            // For,
            parse_rule!(None, None, Precedence::None),
            // In,
            parse_rule!(None, None, Precedence::None),
            // Function,
            parse_rule!(None, None, Precedence::None),
            // Mut,
            parse_rule!(None, None, Precedence::None),
            // Nil,
            parse_rule!(None, None, Precedence::None),
            // Not,
            parse_rule!(None, None, Precedence::None),
            // Or,
            parse_rule!(None, None, Precedence::None),
            // While,
            parse_rule!(None, None, Precedence::None),
            // Repeat,
            parse_rule!(None, None, Precedence::None),
            // Return,
            parse_rule!(None, None, Precedence::None),
            // Until,
            parse_rule!(None, None, Precedence::None),
            // Require,
            parse_rule!(None, None, Precedence::None),
            // Break,
            parse_rule!(None, None, Precedence::None),
            // Add,
            parse_rule!(
                None,
                Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_binary_op(current_token.unwrap())
                }),
                Precedence::Term
            ),
            // Sub,
            parse_rule!(
                Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_unary_op(current_token.unwrap())
                }),
                Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_binary_op(current_token.unwrap())
                }),
                Precedence::Term
            ),
            // Mul,
            parse_rule!(
                None,
                Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_binary_op(current_token.unwrap())
                }),
                Precedence::Factor
            ),
            // Div,
            parse_rule!(
                None,
                Some(|parser: &mut Parser<'_, R>, current_token| {
                    parser.parse_binary_op(current_token.unwrap())
                }),
                Precedence::Factor
            ),
            // Mod,
            parse_rule!(None, None, Precedence::None),
            // Pow,
            parse_rule!(None, None, Precedence::None),
            // Len,
            parse_rule!(None, None, Precedence::None),
            // BitAnd,
            parse_rule!(None, None, Precedence::None),
            // BitOr,
            parse_rule!(None, None, Precedence::None),
            // BitNot,
            parse_rule!(None, None, Precedence::None),
            // ShiftL,
            parse_rule!(None, None, Precedence::None),
            // ShiftR,
            parse_rule!(None, None, Precedence::None),
            // Idiv,
            parse_rule!(None, None, Precedence::None),
            // Equal,
            parse_rule!(None, None, Precedence::None),
            // NotEq,
            parse_rule!(None, None, Precedence::None),
            // LesEq,
            parse_rule!(None, None, Precedence::None),
            // GreEq,
            parse_rule!(None, None, Precedence::None),
            // Less,
            parse_rule!(None, None, Precedence::None),
            // Greater,
            parse_rule!(None, None, Precedence::None),
            // Assign,
            parse_rule!(None, None, Precedence::None),
            // ParL,
            parse_rule!(
                Some(|parser: &mut Parser<'_, R>, _| parser.parse_grouping()),
                None,
                Precedence::None
            ),
            // ParR,
            parse_rule!(None, None, Precedence::None),
            // CurlyL,
            parse_rule!(None, None, Precedence::None),
            // CurlyR,
            parse_rule!(None, None, Precedence::None),
            // SqurL,
            parse_rule!(None, None, Precedence::None),
            // SqurR,
            parse_rule!(None, None, Precedence::None),
            // DoubColon,
            parse_rule!(None, None, Precedence::None),
            // SemiColon,
            parse_rule!(None, None, Precedence::None),
            // Colon,
            parse_rule!(None, None, Precedence::None),
            // Comma,
            parse_rule!(None, None, Precedence::None),
            // Dot,
            parse_rule!(None, None, Precedence::None),
            // Concat,
            parse_rule!(None, None, Precedence::None),
            // Dots,
            parse_rule!(None, None, Precedence::None),
            // Arrow,
            parse_rule!(None, None, Precedence::None),
            // Int,
            parse_rule!(None, None, Precedence::None),
            // Float,
            parse_rule!(None, None, Precedence::None),
            // String,
            parse_rule!(None, None, Precedence::None),
            // Bool,
            parse_rule!(None, None, Precedence::None),
            // Byte,
            parse_rule!(None, None, Precedence::None),
            // Identifier,
            parse_rule!(None, None, Precedence::None),
            // EoS,
            parse_rule!(None, None, Precedence::None),
        ]
    }
}

// NOTE - Lazy static maybe?
// lazy_static::lazy_static! {
//     static ref RULES: [&'static Option<ParseRule<'static, Box<dyn Read>>>; RULES_COUNT] = {
//         let mut rules = [&None; RULES_COUNT];

//         // Add rules here using local values
//         rules[TokenRule::And as usize] = &Some(ParseRule::new(None, None, Precedence::None));
//         rules[TokenRule::Do as usize] = &Some(ParseRule::new(None, None, Precedence::None));
//         // Add more rules...

//         rules
//     };
// }

// pub fn get_rule<R: Read>(token: TokenRule) -> Option<&'static ParseRule<'static, R>> {
//     RULES[token as usize].as_ref()
// }
