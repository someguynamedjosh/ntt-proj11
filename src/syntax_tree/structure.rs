#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Program {
    pub classes: Vec<Class>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Class {
    pub variables: Vec<ClassVariable>,
    pub subroutines: Vec<ClassSubroutine>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableDeclaration {
    pub name: String,
    pub typ: DataType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    BuiltinInt,
    BuiltinChar,
    BuiltinBool,
    /// Used for class names as data types.
    Other(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Statement {
    Do(Expression),
    Let {
        variable_name: String,
        value: Expression,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Return,
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Vec<Statement>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Identifier(String),
    ArrayAccess {
        base: Box<Expression>,
        index: Box<Expression>,
    },
    PropertyAccess {
        base: Box<Expression>,
        property_name: String,
    },
    UnaryOperation {
        operator: UnaryOperator,
        rhs: Box<Expression>,
    },
    BinaryOperation {
        lhs: Box<Expression>,
        operator: BinaryOperator,
        rhs: Box<Expression>,
    },
    SubroutineCall {
        subroutine: Box<Expression>,
        args: Vec<Expression>,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Negate,
    BitwiseNot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Equal,
    BitwiseAnd,
    BitwiseOr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClassVariableType {
    Static,
    Field,
}
// The variant names are specific enough and the enum name is long enough that this works well to
// do it by default.
pub use ClassVariableType::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassVariable {
    pub name: String,
    // 'type' is a rust keyword.
    pub typ: ClassVariableType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClassSubroutineType {
    StaticFunction,
    Method,
    Constructor,
}
pub use ClassSubroutineType::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassSubroutine {
    pub name: String,
    // 'type' is a rust keyword.
    pub typ: ClassSubroutineType,
    pub parameters: Vec<VariableDeclaration>,
    pub local_variables: Vec<VariableDeclaration>,
    pub body: Vec<Statement>,
}
