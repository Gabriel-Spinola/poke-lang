use macros::ConvertToTokenRule;

#[derive(Debug, Clone, ConvertToTokenRule)]
pub enum Token {
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
    Int { value: i32 },
    Float { value: f64 },
    String { value: String },
    Bool { value: bool },
    Byte { value: u8 },

    Identifier(String),

    // End of line
    EoS,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int { value: l_value }, Self::Int { value: r_value }) => l_value == r_value,
            (Self::Float { value: l_value }, Self::Float { value: r_value }) => l_value == r_value,
            (Self::String { value: l_value }, Self::String { value: r_value }) => {
                l_value == r_value
            }
            (Self::Bool { value: l_value }, Self::Bool { value: r_value }) => l_value == r_value,
            (Self::Byte { value: l_value }, Self::Byte { value: r_value }) => l_value == r_value,
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

// REVIEW - Consider convert to a impl of Token
#[repr(u8)]
#[derive(Debug)]
pub enum TokenRule {
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
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Len,
    BitAnd,
    BitOr,
    BitNot,
    ShiftL,
    ShiftR,
    Idiv,
    Equal,
    NotEq,
    LesEq,
    GreEq,
    Less,
    Greater,
    Assign,
    ParL,
    ParR,
    CurlyL,
    CurlyR,
    SqurL,
    SqurR,
    DoubColon,
    SemiColon,
    Colon,
    Comma,
    Dot,
    Concat,
    Dots,
    Arrow,
    Int,
    Float,
    String,
    Bool,
    Byte,

    Identifier,
    EoS,
}

#[cfg(test)]
pub const TOKENS_MOCK: [&Token; 52] = [
    // Testing operator and comments
    &Token::Sub,
    &Token::Add,
    &Token::Mul,
    &Token::Mod,
    &Token::Pow,
    &Token::Len,
    &Token::BitAnd,
    &Token::BitOr,
    &Token::ParL,
    &Token::ParR,
    &Token::CurlyL,
    &Token::CurlyR,
    &Token::SqurL,
    &Token::SqurR,
    &Token::SemiColon,
    &Token::Comma,
    &Token::Colon,
    &Token::Colon,
    &Token::Div,
    &Token::Div,
    &Token::Equal,
    &Token::Equal,
    &Token::NotEq,
    &Token::NotEq,
    &Token::LesEq,
    &Token::ShiftL,
    &Token::Less,
    &Token::Greater,
    &Token::GreEq,
    &Token::ShiftR,
    &Token::ShiftR,
    &Token::Arrow,
    // Testing keywords
    &Token::Mut,
    &Token::Require,
    &Token::And,
    &Token::Break,
    &Token::Do,
    &Token::Else,
    &Token::ElseIf,
    &Token::End,
    &Token::For,
    &Token::Function,
    &Token::If,
    &Token::In,
    &Token::Nil,
    &Token::Not,
    &Token::Or,
    &Token::Repeat,
    &Token::Return,
    &Token::Then,
    &Token::Until,
    &Token::While,
];
