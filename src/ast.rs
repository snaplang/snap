/// The root of a parsed .sp file
#[derive(Debug, Clone)]
pub struct Program {
    pub imports: Vec<Import>,
    pub globals: Vec<GlobalVar>,
    pub stage: Option<StageNode>,
    pub sprites: Vec<SpriteNode>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct GlobalVar {
    pub name: String,
    #[allow(dead_code)] // Will be used for type checking in future
    pub var_type: VarType,
    pub initial_value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Int,
    Float,
    Bool,
    String,
}

#[derive(Debug, Clone)]
pub struct StageNode {
    #[allow(dead_code)] // Will be used for custom backdrops in future
    pub backdrops: Vec<String>,
    pub code: Option<CodeBlock>,
}

#[derive(Debug, Clone)]
pub struct SpriteNode {
    pub name: String,
    #[allow(dead_code)] // Will be used for custom costumes in future
    pub costumes: Vec<String>,
    pub position: Option<(f64, f64)>,
    pub size: Option<f64>,
    pub code: Option<CodeBlock>,
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub event_handlers: Vec<EventHandler>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct EventHandler {
    pub event: Event,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Event {
    GreenFlag,
    KeyPressed(String),
    Clicked,
    Broadcast(String),
    BackdropSwitch(String),
    CloneStart,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: VarType,
}

#[derive(Debug, Clone)]
pub enum Statement {
    /// A block call like `motion::Move(10)` or `looks::Say("hi")`
    BlockCall(BlockCall),

    /// Variable assignment: `set score = 10`
    SetVariable { name: String, value: Expression },

    /// Variable change: `change score by 1`
    ChangeVariable { name: String, value: Expression },

    /// If statement
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },

    /// Forever loop (from control::Forever)
    Forever { body: Vec<Statement> },

    /// Repeat loop (from control::Repeat)
    Repeat {
        times: Expression,
        body: Vec<Statement>,
    },

    /// Repeat until loop
    RepeatUntil {
        condition: Expression,
        body: Vec<Statement>,
    },

    /// Wait seconds
    Wait { duration: Expression },

    /// Stop
    Stop {
        mode: String, // "all", "this script", "other scripts in sprite"
    },

    /// Create clone
    CreateClone {
        target: String, // "_myself_" or sprite name
    },

    /// Delete this clone
    DeleteClone,

    /// Custom function call
    FunctionCall { name: String, args: Vec<Expression> },
}

#[derive(Debug, Clone)]
pub struct BlockCall {
    pub category: String,
    pub block_name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    /// Integer literal
    IntLiteral(i64),

    /// Float literal
    FloatLiteral(f64),

    /// String literal
    StringLiteral(String),

    /// Boolean literal
    BoolLiteral(bool),

    /// Variable reference
    Variable(String),

    /// Binary operation
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },

    /// Unary operation
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
    },

    /// A reporter block like `sensing::MouseX` or `operators::Random(1, 10)`
    ReporterCall(BlockCall),

    /// Unit wrapper like `units::Sec(1)`
    UnitValue {
        #[allow(dead_code)] // Will be used for unit validation in future
        unit: String,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Neg,
}
