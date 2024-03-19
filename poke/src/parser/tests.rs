use std::{fs::File, io::BufReader};

use crate::parser::tokens::{Token, TOKENS_MOCK};

use super::Lexer;

#[test]
fn test_lexer_tokens() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let relative_path = "..\\tests\\lang\\test_lexer.poke";
    let file_path = current_dir.join(relative_path);

    let file = File::open(file_path).expect("Failed to open test file");
    let mut lexer = Lexer::new(BufReader::new(file));

    let mut previus_line = lexer.current_line;
    lexer.expect(Token::Sub);

    for i in 1..TOKENS_MOCK.len() {
        println!("{i}");
        println!("AAAAAAAAA {:?}", TOKENS_MOCK[i]);
        // +
        lexer.expect(TOKENS_MOCK[i].clone());

        // *
        let token = lexer.advance();
        if token == Token::EoS {
            println!("END STREAM");

            break;
        }

        match previus_line {
            // lex line starts from 0.
            0 => print!("{:04} ", lexer.current_line + 1),

            _ if previus_line == lexer.current_line + 1 => print!("  |  "),
            _ => print!("{:04} ", lexer.current_line + 1),
        }

        previus_line = lexer.current_line + 1;
    }
}
