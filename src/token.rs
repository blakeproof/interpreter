#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal, //"ILLEGAL"
    Eof, //"EOF"
    Blank, // " "

    // Identifiers + literals
    Ident{name : String}, //"IDENT"
    Int(i64), //"INT"
    String(String),

    // Operators
    Assign, //"="
    Plus, //"+"
    Minus, //"-"
    Bang, //"!"
    Asterisk, //"*"
    Slash, //"/"

    Lt, //"<"
    Gt, //">"
    LtEq, // "<="
    GtEq, // ">="

    Eq, //"=="
    NotEq, //"!="

    // Delimiters
    Comma, //","
    Semicolon, //";"

    Lparen, // "("
    Rparen, //")"
    Lbrace, //"{"
    Rbrace, //"}"

    // Keywords
    Function, //"FUNCTION"
    Let, //"LET"
    True, //"TRUE"
    False, //"FALSE"
    If, //"IF"
    Else, //"ELSE"
    Return, //"RETURN"
}
