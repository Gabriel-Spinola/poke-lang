// TODO - Implement lexical error result type
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    And,
    Do,
    Then,
    If,
    Else,
    ElseIf,
    End,
    False,
    True,
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

    //      ;        :       ,      .    <>     ..
    SemiColon,
    Colon,
    Comma,
    Dot,
    Concat,
    Dots,
    Arrow,

    // Data types, (refers to to their actual value no keywords)
    Int { value: i32 },
    Float { value: f32 },
    String { value: String },
    Bool { value: bool },
    Byte { value: u8 },

    Identifier(String),
    Numbers,

    // End of line
    EoS,
}
