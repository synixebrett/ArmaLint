use super::Node;

#[derive(Debug, Clone, PartialEq, AsStaticStr)]
pub enum Statement {
    Config(Vec<Node>),
    Array(Vec<Node>),
    Float(f32),
    Integer(i32),
    Str(String),
    Bool(bool),
    Char(char),
    Unquoted(Vec<Node>),
    Property {
        ident: Box<Node>,
        value: Box<Node>,
        expand: bool,
    },
    Class {
        ident: Box<Node>,
        extends: Option<Box<Node>>,
        props: Vec<Node>,
    },
    ClassDef(Box<Node>),
    ClassDelete(Box<Node>),
    Ident(String),
    IdentArray(String),

    // Special
    FILE,
    LINE,

    // Directives
    Define {
        ident: String,
        value: Option<Box<Node>>,
    },
    DefineMacro {
        ident: String,
        args: Vec<String>,
        value: Box<Node>,
    },
    MacroCall {
        ident: String,
        args: Vec<Node>,
    },
    MacroBody(String),
    MacroCallArg(Vec<Node>),
    Undefine(String),
    IfDef {
        ident: String,
        positive: Vec<Node>,
        negative: Option<Vec<Node>>,
    },

    // Internal
    Processed(Box<Statement>, Box<Statement>),
    InternalStr(String),
    Undefined(String, Box<Statement>),
    FlagAsIdent(String, Box<Node>),
    // Definition, Original
    Defined(Box<Node>, Box<Node>),
    InvalidCall(String, Box<Statement>),
    Inserted(Vec<Node>),
    Gone,

    // Warnings & Errors
    NonUppercaseDefine(Box<Statement>),
    Redefine(String, Box<Statement>, Box<Option<Node>>),
}
