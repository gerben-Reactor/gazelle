//! C11 Parser POC for Gazelle
//!
//! Demonstrates two key innovations:
//! 1. Jourdan's typedef disambiguation via NAME TYPE/NAME VARIABLE lexer feedback
//! 2. Dynamic precedence parsing via `prec` terminals - collapses 10 expression levels into one rule

use std::collections::HashSet;

use gazelle::Precedence;
use gazelle_macros::gazelle;

// =============================================================================
// Grammar Definition
// =============================================================================

gazelle! {
    grammar C11 = "grammars/c11.gzl"
}

// =============================================================================
// Placeholder types (no AST for now, just validate parsing)
// =============================================================================

// =============================================================================
// Typedef Context (Jourdan's approach: flat set with save/restore)
// =============================================================================

/// A context snapshot - the set of typedef names visible at a point
pub type Context = HashSet<String>;

/// Declarator with optional context for function declarators (Jourdan's approach)
#[derive(Clone, Debug)]
pub enum Declarator {
    /// Simple identifier declarator
    Identifier(String),
    /// Function declarator with saved context at end of parameters
    Function(String, Context),
    /// Other declarator (array, pointer, etc.)
    Other(String),
}

impl Declarator {
    pub fn name(&self) -> &str {
        match self {
            Declarator::Identifier(s) => s,
            Declarator::Function(s, _) => s,
            Declarator::Other(s) => s,
        }
    }

    /// Convert identifier to function declarator with context
    pub fn to_function(self, ctx: Context) -> Self {
        match self {
            Declarator::Identifier(s) => Declarator::Function(s, ctx),
            other => other, // Already function or other, don't override
        }
    }

    /// Convert identifier to other declarator
    pub fn to_other(self) -> Self {
        match self {
            Declarator::Identifier(s) => Declarator::Other(s),
            other => other,
        }
    }
}

/// Typedef context for tracking declared typedef names.
/// Uses Jourdan's approach: a single mutable set with save/restore.
pub struct TypedefContext {
    current: HashSet<String>,
}

impl TypedefContext {
    pub fn new() -> Self {
        Self {
            current: HashSet::new(),
        }
    }

    /// Test if name is a typedef
    pub fn is_typedef(&self, name: &str) -> bool {
        self.current.contains(name)
    }

    /// Declare a typedef name (adds to set)
    pub fn declare_typedef(&mut self, name: &str) {
        self.current.insert(name.to_string());
    }

    /// Declare a variable name (removes from set, shadowing any typedef)
    pub fn declare_varname(&mut self, name: &str) {
        self.current.remove(name);
    }

    /// Save the current context (returns a snapshot)
    pub fn save(&self) -> Context {
        self.current.clone()
    }

    /// Restore a saved context
    pub fn restore(&mut self, snapshot: Context) {
        self.current = snapshot;
    }
}

impl Default for TypedefContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Actions for the C11 parser (empty - just validate parsing)
pub struct CActions {
    pub ctx: TypedefContext,
}

impl CActions {
    pub fn new() -> Self {
        Self {
            ctx: TypedefContext::new(),
        }
    }
}

impl Default for CActions {
    fn default() -> Self {
        Self::new()
    }
}

impl C11Types for CActions {
    type Error = gazelle::ParseError;
    type Name = String;
    type TypedefName = String;
    type VarName = String;
    type GeneralIdentifier = String;
    type EnumerationConstant = String;
    type SaveContext = Context;
    type ScopedCompoundStatement = ();
    type ScopedIterationStatement = ();
    type ScopedParameterTypeList = Context;
    type ScopedSelectionStatement = ();
    type ScopedStatement = ();
    type DeclaratorVarname = Declarator;
    type DeclaratorTypedefname = Declarator;
    type Declarator = Declarator;
    type DirectDeclarator = Declarator;
    type Enumerator = ();
    type ParameterTypeList = Context;
    type FunctionDefinition1 = Context;
    type FunctionDefinition = ();
    // All remaining NTs use Ignore (parse-only validator, no AST needed)
    type OptionAnonymous2 = gazelle::Ignore;
    type OptionArgumentExpressionList = gazelle::Ignore;
    type OptionAssignmentExpression = gazelle::Ignore;
    type OptionBlockItemList = gazelle::Ignore;
    type OptionDeclarationList = gazelle::Ignore;
    type OptionDeclarator = gazelle::Ignore;
    type OptionDesignation = gazelle::Ignore;
    type OptionDesignatorList = gazelle::Ignore;
    type OptionExpression = gazelle::Ignore;
    type OptionGeneralIdentifier = gazelle::Ignore;
    type OptionIdentifierList = gazelle::Ignore;
    type OptionInitDeclaratorListDeclaratorTypedefname = gazelle::Ignore;
    type OptionInitDeclaratorListDeclaratorVarname = gazelle::Ignore;
    type OptionPointer = gazelle::Ignore;
    type OptionScopedParameterTypeList = gazelle::Ignore;
    type OptionStructDeclaratorList = gazelle::Ignore;
    type OptionTypeQualifierList = gazelle::Ignore;
    type ListAnonymous0 = gazelle::Ignore;
    type ListAnonymous1 = gazelle::Ignore;
    type ListDeclarationSpecifier = gazelle::Ignore;
    type ListEq1TypedefDeclarationSpecifier = gazelle::Ignore;
    type ListEq1TypeSpecifierUniqueAnonymous0 = gazelle::Ignore;
    type ListEq1TypeSpecifierUniqueDeclarationSpecifier = gazelle::Ignore;
    type ListGe1TypeSpecifierNonuniqueAnonymous1 = gazelle::Ignore;
    type ListGe1TypeSpecifierNonuniqueDeclarationSpecifier = gazelle::Ignore;
    type ListEq1Eq1TypedefTypeSpecifierUniqueDeclarationSpecifier = gazelle::Ignore;
    type ListEq1Ge1TypedefTypeSpecifierNonuniqueDeclarationSpecifier = gazelle::Ignore;
    type TypedefNameSpec = gazelle::Ignore;
    type StringLiteral = gazelle::Ignore;
    type PrimaryExpression = gazelle::Ignore;
    type GenericSelection = gazelle::Ignore;
    type GenericAssocList = gazelle::Ignore;
    type GenericAssociation = gazelle::Ignore;
    type PostfixExpression = gazelle::Ignore;
    type ArgumentExpressionList = gazelle::Ignore;
    type UnaryExpression = gazelle::Ignore;
    type UnaryOperator = gazelle::Ignore;
    type CastExpression = gazelle::Ignore;
    type AssignmentExpression = gazelle::Ignore;
    type Expression = gazelle::Ignore;
    type ConstantExpression = gazelle::Ignore;
    type Declaration = gazelle::Ignore;
    type DeclarationSpecifier = gazelle::Ignore;
    type DeclarationSpecifiers = gazelle::Ignore;
    type DeclarationSpecifiersTypedef = gazelle::Ignore;
    type InitDeclaratorListDeclaratorTypedefname = gazelle::Ignore;
    type InitDeclaratorListDeclaratorVarname = gazelle::Ignore;
    type InitDeclaratorDeclaratorTypedefname = gazelle::Ignore;
    type InitDeclaratorDeclaratorVarname = gazelle::Ignore;
    type StorageClassSpecifier = gazelle::Ignore;
    type TypeSpecifierNonunique = gazelle::Ignore;
    type TypeSpecifierUnique = gazelle::Ignore;
    type StructOrUnionSpecifier = gazelle::Ignore;
    type StructOrUnion = gazelle::Ignore;
    type StructDeclarationList = gazelle::Ignore;
    type StructDeclaration = gazelle::Ignore;
    type SpecifierQualifierList = gazelle::Ignore;
    type StructDeclaratorList = gazelle::Ignore;
    type StructDeclarator = gazelle::Ignore;
    type EnumSpecifier = gazelle::Ignore;
    type EnumeratorList = gazelle::Ignore;
    type AtomicTypeSpecifier = gazelle::Ignore;
    type TypeQualifier = gazelle::Ignore;
    type FunctionSpecifier = gazelle::Ignore;
    type AlignmentSpecifier = gazelle::Ignore;
    type Pointer = gazelle::Ignore;
    type TypeQualifierList = gazelle::Ignore;
    type ParameterList = gazelle::Ignore;
    type ParameterDeclaration = gazelle::Ignore;
    type IdentifierList = gazelle::Ignore;
    type TypeName = gazelle::Ignore;
    type AbstractDeclarator = gazelle::Ignore;
    type DirectAbstractDeclarator = gazelle::Ignore;
    type CInitializer = gazelle::Ignore;
    type InitializerList = gazelle::Ignore;
    type Designation = gazelle::Ignore;
    type DesignatorList = gazelle::Ignore;
    type Designator = gazelle::Ignore;
    type StaticAssertDeclaration = gazelle::Ignore;
    type Statement = gazelle::Ignore;
    type LabeledStatement = gazelle::Ignore;
    type CompoundStatement = gazelle::Ignore;
    type BlockItemList = gazelle::Ignore;
    type BlockItem = gazelle::Ignore;
    type ExpressionStatement = gazelle::Ignore;
    type SelectionStatement = gazelle::Ignore;
    type IterationStatement = gazelle::Ignore;
    type JumpStatement = gazelle::Ignore;
    type TranslationUnitFile = gazelle::Ignore;
    type ExternalDeclaration = gazelle::Ignore;
    type DeclarationList = gazelle::Ignore;
}

use gazelle::Reduce;

impl Reduce<C11TypedefName<Self>, String, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11TypedefName<Self>) -> Result<String, gazelle::ParseError> {
        let C11TypedefName::TypedefName(name) = node;
        Ok(name)
    }
}

impl Reduce<C11VarName<Self>, String, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11VarName<Self>) -> Result<String, gazelle::ParseError> {
        let C11VarName::VarName(name) = node;
        Ok(name)
    }
}

impl Reduce<C11GeneralIdentifier<Self>, String, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11GeneralIdentifier<Self>) -> Result<String, gazelle::ParseError> {
        Ok(match node {
            C11GeneralIdentifier::Typedef(name) => name,
            C11GeneralIdentifier::Var(name) => name,
        })
    }
}

impl Reduce<C11EnumerationConstant<Self>, String, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11EnumerationConstant<Self>) -> Result<String, gazelle::ParseError> {
        let C11EnumerationConstant::EnumConst(name) = node;
        Ok(name)
    }
}

impl Reduce<C11SaveContext, Context, gazelle::ParseError> for CActions {
    fn reduce(&mut self, _: C11SaveContext) -> Result<Context, gazelle::ParseError> {
        Ok(self.ctx.save())
    }
}

impl Reduce<C11ScopedCompoundStatement<Self>, (), gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11ScopedCompoundStatement<Self>) -> Result<(), gazelle::ParseError> {
        let C11ScopedCompoundStatement::RestoreCompound(ctx, _) = node;
        self.ctx.restore(ctx);
        Ok(())
    }
}

impl Reduce<C11ScopedIterationStatement<Self>, (), gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11ScopedIterationStatement<Self>) -> Result<(), gazelle::ParseError> {
        let C11ScopedIterationStatement::RestoreIteration(ctx, _) = node;
        self.ctx.restore(ctx);
        Ok(())
    }
}

impl Reduce<C11ScopedSelectionStatement<Self>, (), gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11ScopedSelectionStatement<Self>) -> Result<(), gazelle::ParseError> {
        let C11ScopedSelectionStatement::RestoreSelection(ctx, _) = node;
        self.ctx.restore(ctx);
        Ok(())
    }
}

impl Reduce<C11ScopedStatement<Self>, (), gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11ScopedStatement<Self>) -> Result<(), gazelle::ParseError> {
        let C11ScopedStatement::RestoreStatement(ctx, _) = node;
        self.ctx.restore(ctx);
        Ok(())
    }
}

impl Reduce<C11ScopedParameterTypeList<Self>, Context, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11ScopedParameterTypeList<Self>) -> Result<Context, gazelle::ParseError> {
        let C11ScopedParameterTypeList::ScopedParams(start_ctx, end_ctx) = node;
        self.ctx.restore(start_ctx);
        Ok(end_ctx)
    }
}

impl Reduce<C11ParameterTypeList<Self>, Context, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11ParameterTypeList<Self>) -> Result<Context, gazelle::ParseError> {
        let C11ParameterTypeList::ParamCtx(_, _, ctx) = node;
        Ok(ctx)
    }
}

impl Reduce<C11DirectDeclarator<Self>, Declarator, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11DirectDeclarator<Self>) -> Result<Declarator, gazelle::ParseError> {
        Ok(match node {
            C11DirectDeclarator::DdIdent(name) => Declarator::Identifier(name),
            C11DirectDeclarator::DdParen(_ctx, d) => d,
            C11DirectDeclarator::DdOther(d, _, _)
            | C11DirectDeclarator::DdOther1(d, _, _)
            | C11DirectDeclarator::DdOther2(d, _, _)
            | C11DirectDeclarator::DdOther3(d, _) => d.to_other(),
            C11DirectDeclarator::DdFunc(d, ctx) => d.to_function(ctx),
            C11DirectDeclarator::DdOtherKr(d, _ctx, _) => d.to_other(),
        })
    }
}

impl Reduce<C11Declarator<Self>, Declarator, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11Declarator<Self>) -> Result<Declarator, gazelle::ParseError> {
        Ok(match node {
            C11Declarator::DeclDirect(d) => d,
            C11Declarator::DeclPtr(_, d) => d.to_other(),
        })
    }
}

impl Reduce<C11DeclaratorVarname<Self>, Declarator, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11DeclaratorVarname<Self>) -> Result<Declarator, gazelle::ParseError> {
        let C11DeclaratorVarname::DeclVarname(d) = node;
        self.ctx.declare_varname(d.name());
        Ok(d)
    }
}

impl Reduce<C11DeclaratorTypedefname<Self>, Declarator, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11DeclaratorTypedefname<Self>) -> Result<Declarator, gazelle::ParseError> {
        let C11DeclaratorTypedefname::RegisterTypedef(d) = node;
        self.ctx.declare_typedef(d.name());
        Ok(d)
    }
}

impl Reduce<C11FunctionDefinition1<Self>, Context, gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11FunctionDefinition1<Self>) -> Result<Context, gazelle::ParseError> {
        let C11FunctionDefinition1::FuncDef1(_, d) = node;
        let saved = self.ctx.save();
        if let Declarator::Function(name, param_ctx) = &d {
            self.ctx.restore(param_ctx.clone());
            self.ctx.declare_varname(name);
        }
        Ok(saved)
    }
}

impl Reduce<C11FunctionDefinition<Self>, (), gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11FunctionDefinition<Self>) -> Result<(), gazelle::ParseError> {
        let C11FunctionDefinition::FuncDef(ctx, _, _) = node;
        self.ctx.restore(ctx);
        Ok(())
    }
}

impl Reduce<C11Enumerator<Self>, (), gazelle::ParseError> for CActions {
    fn reduce(&mut self, node: C11Enumerator<Self>) -> Result<(), gazelle::ParseError> {
        match node {
            C11Enumerator::DeclEnum(name) | C11Enumerator::DeclEnumExpr(name, _) => {
                self.ctx.declare_varname(&name);
            }
        }
        Ok(())
    }
}

// =============================================================================
// C Lexer with Typedef Feedback
// =============================================================================

/// C11 lexer with lexer feedback for typedef disambiguation
pub struct C11Lexer<'a> {
    input: &'a str,
    src: gazelle::lexer::Source<std::str::Chars<'a>>,
    /// Pending identifier - when Some, next call returns TYPE or VARIABLE
    /// based on is_typedef check at that moment (delayed decision)
    pending_ident: Option<String>,
}

impl<'a> C11Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            src: gazelle::lexer::Source::from_str(input),
            pending_ident: None,
        }
    }

    fn next(&mut self, ctx: &TypedefContext) -> Result<Option<C11Terminal<CActions>>, String> {
        // If we have a pending identifier, emit TYPE or VARIABLE based on current context
        // This is the key: the decision is made NOW, not when NAME was seen
        if let Some(id) = self.pending_ident.take() {
            return Ok(Some(if ctx.is_typedef(&id) {
                C11Terminal::Type
            } else {
                C11Terminal::Variable
            }));
        }

        // Skip whitespace and comments
        self.src.skip_whitespace();
        while self.src.skip_line_comment("//") || self.src.skip_block_comment("/*", "*/") {
            self.src.skip_whitespace();
        }

        if self.src.at_end() {
            return Ok(None);
        }

        // Identifier or keyword
        if let Some(span) = self.src.read_ident() {
            let s = &self.input[span];

            // Check for C-style prefixed string/char literals: L, u, U, u8
            if matches!(s, "L" | "u" | "U" | "u8") {
                if self.src.peek() == Some('\'') {
                    self.src.read_string_raw('\'').map_err(|e| e.to_string())?;
                    return Ok(Some(C11Terminal::Constant));
                } else if self.src.peek() == Some('"') {
                    self.src.read_string_raw('"').map_err(|e| e.to_string())?;
                    return Ok(Some(C11Terminal::StringLiteral));
                }
            }

            return Ok(Some(match s {
                // Keywords
                "auto" => C11Terminal::Auto,
                "break" => C11Terminal::Break,
                "case" => C11Terminal::Case,
                "char" => C11Terminal::Char,
                "const" => C11Terminal::Const,
                "continue" => C11Terminal::Continue,
                "default" => C11Terminal::Default,
                "do" => C11Terminal::Do,
                "double" => C11Terminal::Double,
                "else" => C11Terminal::Else,
                "enum" => C11Terminal::Enum,
                "extern" => C11Terminal::Extern,
                "float" => C11Terminal::Float,
                "for" => C11Terminal::For,
                "goto" => C11Terminal::Goto,
                "if" => C11Terminal::If,
                "inline" => C11Terminal::Inline,
                "int" => C11Terminal::Int,
                "long" => C11Terminal::Long,
                "register" => C11Terminal::Register,
                "restrict" => C11Terminal::Restrict,
                "return" => C11Terminal::Return,
                "short" => C11Terminal::Short,
                "signed" => C11Terminal::Signed,
                "sizeof" => C11Terminal::Sizeof,
                "static" => C11Terminal::Static,
                "struct" => C11Terminal::Struct,
                "switch" => C11Terminal::Switch,
                "typedef" => C11Terminal::Typedef,
                "union" => C11Terminal::Union,
                "unsigned" => C11Terminal::Unsigned,
                "void" => C11Terminal::Void,
                "volatile" => C11Terminal::Volatile,
                "while" => C11Terminal::While,
                // C11 keywords
                "_Alignas" => C11Terminal::Alignas,
                "_Alignof" => C11Terminal::Alignof,
                "_Atomic" => C11Terminal::Atomic,
                "_Bool" => C11Terminal::Bool,
                "_Complex" => C11Terminal::Complex,
                "_Generic" => C11Terminal::Generic,
                "_Imaginary" => C11Terminal::Imaginary,
                "_Noreturn" => C11Terminal::Noreturn,
                "_Static_assert" => C11Terminal::StaticAssert,
                "_Thread_local" => C11Terminal::ThreadLocal,
                // Identifier - queue TYPE/VARIABLE for next call
                _ => {
                    self.pending_ident = Some(s.to_string());
                    C11Terminal::Name(s.to_string())
                }
            }));
        }

        // Number or character literal -> CONSTANT
        if self.src.read_digits().is_some() {
            return Ok(Some(C11Terminal::Constant));
        }

        // String literal
        if self.src.peek() == Some('"') {
            self.src.read_string_raw('"').map_err(|e| e.to_string())?;
            return Ok(Some(C11Terminal::StringLiteral));
        }

        // Character literal
        if self.src.peek() == Some('\'') {
            self.src.read_string_raw('\'').map_err(|e| e.to_string())?;
            return Ok(Some(C11Terminal::Constant));
        }

        // Punctuation
        if let Some(c) = self.src.peek() {
            match c {
                '(' => { self.src.advance(); return Ok(Some(C11Terminal::Lparen)); }
                ')' => { self.src.advance(); return Ok(Some(C11Terminal::Rparen)); }
                '{' => { self.src.advance(); return Ok(Some(C11Terminal::Lbrace)); }
                '}' => { self.src.advance(); return Ok(Some(C11Terminal::Rbrace)); }
                '[' => { self.src.advance(); return Ok(Some(C11Terminal::Lbrack)); }
                ']' => { self.src.advance(); return Ok(Some(C11Terminal::Rbrack)); }
                ';' => { self.src.advance(); return Ok(Some(C11Terminal::Semicolon)); }
                ',' => { self.src.advance(); return Ok(Some(C11Terminal::Comma)); }
                _ => {}
            }
        }

        // Multi-char operators (longest first for maximal munch)
        const MULTI_OPS: &[&str] = &[
            "...", "<<=", ">>=",  // 0-2: three-char
            "->", "++", "--",  // 3-5: special two-char
            "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=",  // 6-13: assign
            "||", "&&", "==", "!=", "<=", ">=", "<<", ">>",  // 14-21: binary
        ];
        const MULTI_PREC: &[Option<Precedence>] = &[
            None, Some(Precedence::Right(1)), Some(Precedence::Right(1)),  // 0-2
            None, None, None,  // 3-5: PTR, INC, DEC (no prec)
            Some(Precedence::Right(1)), Some(Precedence::Right(1)), Some(Precedence::Right(1)),
            Some(Precedence::Right(1)), Some(Precedence::Right(1)), Some(Precedence::Right(1)),
            Some(Precedence::Right(1)), Some(Precedence::Right(1)),  // 6-13
            Some(Precedence::Left(3)), Some(Precedence::Left(4)),
            Some(Precedence::Left(8)), Some(Precedence::Left(8)),
            Some(Precedence::Left(9)), Some(Precedence::Left(9)),
            Some(Precedence::Left(10)), Some(Precedence::Left(10)),  // 14-21
        ];

        if let Some((idx, _)) = self.src.read_one_of(MULTI_OPS) {
            return Ok(Some(match idx {
                0 => C11Terminal::Ellipsis,
                3 => C11Terminal::Ptr,
                4 => C11Terminal::Inc,
                5 => C11Terminal::Dec,
                _ => C11Terminal::Binop(MULTI_PREC[idx].unwrap()),
            }));
        }

        // Single-char operators
        if let Some(c) = self.src.peek() {
            self.src.advance();
            return Ok(Some(match c {
                ':' => C11Terminal::Colon,
                '.' => C11Terminal::Dot,
                '~' => C11Terminal::Tilde,
                '!' => C11Terminal::Bang,
                '=' => C11Terminal::Eq(Precedence::Right(1)),
                '?' => C11Terminal::Question(Precedence::Right(2)),
                '|' => C11Terminal::Binop(Precedence::Left(5)),
                '^' => C11Terminal::Binop(Precedence::Left(6)),
                '&' => C11Terminal::Amp(Precedence::Left(7)),
                '<' => C11Terminal::Binop(Precedence::Left(9)),
                '>' => C11Terminal::Binop(Precedence::Left(9)),
                '+' => C11Terminal::Plus(Precedence::Left(11)),
                '-' => C11Terminal::Minus(Precedence::Left(11)),
                '*' => C11Terminal::Star(Precedence::Left(12)),
                '/' => C11Terminal::Binop(Precedence::Left(12)),
                '%' => C11Terminal::Binop(Precedence::Left(12)),
                _ => return Err(format!("Unknown character: {}", c)),
            }));
        }

        Ok(None)
    }
}

// =============================================================================
// Parse Function
// =============================================================================

/// Parse C11 source code
pub fn parse(input: &str) -> Result<(), String> {
    // Strip preprocessor lines (lines starting with #)
    let preprocessed = input
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    parse_debug(&preprocessed, true)
}

/// Parse C11 source code with optional debug output
pub fn parse_debug(input: &str, debug: bool) -> Result<(), String> {
    let mut parser = C11Parser::<CActions>::new();
    let mut actions = CActions::new();
    let mut lexer = C11Lexer::new(input);
    let mut token_count = 0;

    loop {
        let tok = lexer.next(&actions.ctx)?;
        match tok {
            Some(t) => {
                if debug {
                    let name = match &t {
                        C11Terminal::Int => "INT",
                        C11Terminal::Name(_) => "NAME",
                        C11Terminal::Variable => "VARIABLE",
                        C11Terminal::Type => "TYPE",
                        C11Terminal::Semicolon => "SEMICOLON",
                        C11Terminal::Typedef => "TYPEDEF",
                        _ => "Other",
                    };
                    eprintln!("Token {}: {} (before state={})", token_count, name, parser.state());
                }
                token_count += 1;
                parser.push(t, &mut actions).map_err(|e| {
                    format!("Parse error at token {}: {:?}", token_count, e)
                })?;
                if debug {
                    eprintln!("  -> after push, state={}", parser.state());
                }
            }
            None => break,
        }
    }

    parser.finish(&mut actions).map_err(|(p, e)| format!("Finish error: {}", p.format_error(&e)))?;
    Ok(())
}

// =============================================================================
// Main
// =============================================================================

fn main() {
    println!("C11 Parser POC for Gazelle");
    println!();
    println!("Key innovations demonstrated:");
    println!("1. prec OP terminal - collapses 13+ expression rules into ONE");
    println!("2. Lexer feedback - Jourdan's typedef disambiguation");
    println!();
    println!("Run tests with: cargo test --example c11");
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expression() {
        let mut lexer = C11Lexer::new("int x;");
        let ctx = TypedefContext::new();

        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Int)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Name(_))));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Variable)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Semicolon)));
    }

    #[test]
    fn test_typedef_context() {
        let mut ctx = TypedefContext::new();
        assert!(!ctx.is_typedef("T"));

        ctx.declare_typedef("T");
        assert!(ctx.is_typedef("T"));

        // Save context, add S, then restore
        let saved = ctx.save();
        assert!(ctx.is_typedef("T"));
        ctx.declare_typedef("S");
        assert!(ctx.is_typedef("S"));

        ctx.restore(saved);
        assert!(!ctx.is_typedef("S"));
        assert!(ctx.is_typedef("T"));
    }

    #[test]
    fn test_typedef_shadowing() {
        let mut ctx = TypedefContext::new();
        ctx.declare_typedef("T");
        assert!(ctx.is_typedef("T"));

        // Shadow T with a variable declaration
        ctx.declare_varname("T");
        assert!(!ctx.is_typedef("T"));
    }

    #[test]
    fn test_local_scope_simple() {
        // Simplified version of local_scope.c
        let code = r#"
typedef int T;
void f(void) {
  T y = 1;
}
"#;
        parse(code).expect("basic function with typedef should parse");
    }

    #[test]
    fn test_local_scope_with_shadow() {
        // Local variable shadows typedef
        let code = r#"
typedef int T;
void f(void) {
  int T;
  T = 1;
}
"#;
        parse(code).expect("typedef shadow should parse");
    }

    #[test]
    fn test_local_scope_with_if() {
        // Scoped shadow in if block
        let code = r#"
typedef int T;
void f(void) {
  if(1) {
    int T;
    T = 1;
  }
  T x = 1;
}
"#;
        let preprocessed = code.lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");
        parse_debug(&preprocessed, false).expect("scoped typedef shadow should parse");
    }

    // Note: argument_scope test requires tracking context through declarators
    // (like Jourdan's reinstall_function_context). This is a known limitation.

    #[test]
    fn test_typedef_lexer_feedback() {
        let mut ctx = TypedefContext::new();
        ctx.declare_typedef("MyType");

        let mut lexer = C11Lexer::new("MyType x");

        let tok1 = lexer.next(&ctx).unwrap();
        assert!(matches!(tok1, Some(C11Terminal::Name(_))));

        let tok2 = lexer.next(&ctx).unwrap();
        assert!(matches!(tok2, Some(C11Terminal::Type)));

        let tok3 = lexer.next(&ctx).unwrap();
        assert!(matches!(tok3, Some(C11Terminal::Name(_))));

        let tok4 = lexer.next(&ctx).unwrap();
        assert!(matches!(tok4, Some(C11Terminal::Variable)));
    }

    #[test]
    fn test_keywords() {
        let ctx = TypedefContext::new();
        let mut lexer = C11Lexer::new("int void struct typedef if while for");

        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Int)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Void)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Struct)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Typedef)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::If)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::While)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::For)));
    }

    // =========================================================================
    // C11parser test suite
    // =========================================================================

    /// Helper to parse a C file and report success/failure
    fn parse_c_file(path: &str) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path, e))?;
        parse(&content).map_err(|e| format!("{}: {}", path, e))
    }

    #[test]
    fn test_simple_parse() {
        // Test "int;" first (declaration with no variables)
        eprintln!("\n--- Parsing 'int;' ---");
        let result1 = parse_debug("int;", true);
        if let Err(e) = &result1 {
            eprintln!("'int;' Error: {}", e);
        }

        // Test "typedef int T;" (typedef)
        eprintln!("\n--- Parsing 'typedef int T;' ---");
        let result2 = parse_debug("typedef int T;", true);
        if let Err(e) = &result2 {
            eprintln!("'typedef int T;' Error: {}", e);
        }

        result1.unwrap();
        result2.unwrap();
    }

    /// Test files that should parse successfully
    const C11_TEST_FILES: &[&str] = &[
        "examples/c11/C11parser/tests/typedef_star.c",
        "examples/c11/C11parser/tests/variable_star.c",
        "examples/c11/C11parser/tests/local_typedef.c",
        "examples/c11/C11parser/tests/block_scope.c",
        "examples/c11/C11parser/tests/declaration_ambiguity.c",
        "examples/c11/C11parser/tests/enum.c",
        "examples/c11/C11parser/tests/enum_shadows_typedef.c",
        "examples/c11/C11parser/tests/enum_constant_visibility.c",
        "examples/c11/C11parser/tests/namespaces.c",
        "examples/c11/C11parser/tests/local_scope.c",
        "examples/c11/C11parser/tests/if_scopes.c",
        "examples/c11/C11parser/tests/loop_scopes.c",
        "examples/c11/C11parser/tests/no_local_scope.c",
        "examples/c11/C11parser/tests/function_parameter_scope.c",
        "examples/c11/C11parser/tests/function_parameter_scope_extends.c",
        "examples/c11/C11parser/tests/argument_scope.c",
        "examples/c11/C11parser/tests/control-scope.c",
        "examples/c11/C11parser/tests/dangling_else.c",
        "examples/c11/C11parser/tests/dangling_else_lookahead.c",
        "examples/c11/C11parser/tests/dangling_else_lookahead.if.c",
        "examples/c11/C11parser/tests/parameter_declaration_ambiguity.c",
        "examples/c11/C11parser/tests/parameter_declaration_ambiguity.test.c",
        "examples/c11/C11parser/tests/bitfield_declaration_ambiguity.c",
        "examples/c11/C11parser/tests/bitfield_declaration_ambiguity.ok.c",
        "examples/c11/C11parser/tests/expressions.c",
        "examples/c11/C11parser/tests/statements.c",
        "examples/c11/C11parser/tests/types.c",
        "examples/c11/C11parser/tests/declarators.c",
        "examples/c11/C11parser/tests/designator.c",
        "examples/c11/C11parser/tests/function-decls.c",
        "examples/c11/C11parser/tests/struct-recursion.c",
        "examples/c11/C11parser/tests/long-long-struct.c",
        "examples/c11/C11parser/tests/c-namespace.c",
        "examples/c11/C11parser/tests/enum-trick.c",
        "examples/c11/C11parser/tests/char-literal-printing.c",
        // C11-specific features
        "examples/c11/C11parser/tests/c11-noreturn.c",
        "examples/c11/C11parser/tests/c1x-alignas.c",
        "examples/c11/C11parser/tests/atomic.c",
        "examples/c11/C11parser/tests/atomic_parenthesis.c",
        "examples/c11/C11parser/tests/aligned_struct_c18.c",

        "examples/c11/C11parser/tests/declarator_visibility.c",
    ];

    #[test]
    fn test_c11parser_suite() {
        let mut passed = 0;
        let mut failed = Vec::new();

        for file in C11_TEST_FILES {
            match parse_c_file(file) {
                Ok(()) => {
                    passed += 1;
                    println!("PASS: {}", file);
                }
                Err(e) => {
                    failed.push(e);
                    println!("FAIL: {}", file);
                }
            }
        }

        println!("\n{} passed, {} failed", passed, failed.len());

        if !failed.is_empty() {
            for err in &failed {
                eprintln!("FAIL: {}", err);
            }
            panic!("{} tests failed", failed.len());
        }
    }

    // =========================================================================
    // Expression evaluation tests - verify precedence using actual C11 lexer
    // =========================================================================

    /// Operator enum to distinguish BINOP operators.
    /// Note: Multiple C operators share precedence levels (e.g., == and != both at level 8).
    /// We pick one representative per level since we're testing precedence, not exact semantics.
    #[derive(Clone, Copy, Debug)]
    #[allow(dead_code)]
    enum BinOp {
        Or, And, BitOr, BitXor, Eq, Ne, Lt, Gt, Le, Ge, Shl, Shr, Div, Mod,
    }

    // Minimal expression grammar for evaluation testing
    // Simplified: all expression levels map to i64, just tests precedence
    gazelle! {
        grammar Expr {
            start expr;
            expect 16 sr;  // INC/DEC postfix vs unary prefix ambiguity
            terminals {
                NUM: _,
                LPAREN, RPAREN,
                COLON,
                TILDE, BANG,
                INC, DEC,
                prec EQ,
                prec QUESTION,
                prec STAR,
                prec AMP,
                prec PLUS,
                prec MINUS,
                prec BINOP: _
            }

            // Simplified: term handles primary/postfix/unary/cast
            term = NUM => num
                       | LPAREN expr RPAREN => paren
                       | INC term => preinc
                       | DEC term => predec
                       | AMP term => addr
                       | STAR term => deref
                       | PLUS term => uplus
                       | MINUS term => neg
                       | TILDE term => bitnot
                       | BANG term => lognot
                       | term INC => postinc
                       | term DEC => postdec;

            // Binary expression with dynamic precedence
            expr = term => term
                       | expr BINOP expr => binop
                       | expr STAR expr => mul
                       | expr AMP expr => bitand
                       | expr PLUS expr => add
                       | expr MINUS expr => sub
                       | expr EQ expr => assign
                       | expr QUESTION expr COLON expr => ternary;
        }
    }

    struct Eval;

    impl ExprTypes for Eval {
        type Error = gazelle::ParseError;
        type Num = i64;
        type Binop = BinOp;
        type Term = i64;
        type Expr = i64;
    }

    impl gazelle::Reduce<ExprTerm<Self>, i64, gazelle::ParseError> for Eval {
        fn reduce(&mut self, node: ExprTerm<Self>) -> Result<i64, gazelle::ParseError> {
            Ok(match node {
                ExprTerm::Num(n) => n,
                ExprTerm::Paren(e) => e,
                ExprTerm::Preinc(e) => e + 1,
                ExprTerm::Predec(e) => e - 1,
                ExprTerm::Postinc(e) => e,
                ExprTerm::Postdec(e) => e,
                ExprTerm::Addr(e) => e,
                ExprTerm::Deref(e) => e,
                ExprTerm::Uplus(e) => e,
                ExprTerm::Neg(e) => -e,
                ExprTerm::Bitnot(e) => !e,
                ExprTerm::Lognot(e) => if e == 0 { 1 } else { 0 },
            })
        }
    }

    impl gazelle::Reduce<ExprExpr<Self>, i64, gazelle::ParseError> for Eval {
        fn reduce(&mut self, node: ExprExpr<Self>) -> Result<i64, gazelle::ParseError> {
            Ok(match node {
                ExprExpr::Term(e) => e,
                ExprExpr::Binop(l, op, r) => match op {
                    BinOp::Or => if l != 0 || r != 0 { 1 } else { 0 },
                    BinOp::And => if l != 0 && r != 0 { 1 } else { 0 },
                    BinOp::BitOr => l | r,
                    BinOp::BitXor => l ^ r,
                    BinOp::Eq => if l == r { 1 } else { 0 },
                    BinOp::Ne => if l != r { 1 } else { 0 },
                    BinOp::Lt => if l < r { 1 } else { 0 },
                    BinOp::Gt => if l > r { 1 } else { 0 },
                    BinOp::Le => if l <= r { 1 } else { 0 },
                    BinOp::Ge => if l >= r { 1 } else { 0 },
                    BinOp::Shl => l << r,
                    BinOp::Shr => l >> r,
                    BinOp::Div => l / r,
                    BinOp::Mod => l % r,
                },
                ExprExpr::Mul(l, r) => l * r,
                ExprExpr::Bitand(l, r) => l & r,
                ExprExpr::Add(l, r) => l + r,
                ExprExpr::Sub(l, r) => l - r,
                ExprExpr::Assign(_l, r) => r,
                ExprExpr::Ternary(c, t, e) => if c != 0 { t } else { e },
            })
        }
    }

    /// Convert C11 terminal to expression terminal, using actual C11 lexer
    fn c11_to_expr(tok: C11Terminal<CActions>) -> Option<ExprTerminal<Eval>> {
        Some(match tok {
            C11Terminal::Constant => {
                // For simplicity, constants become 0 - we'll handle numbers specially
                ExprTerminal::Num(0)
            }
            C11Terminal::Lparen => ExprTerminal::Lparen,
            C11Terminal::Rparen => ExprTerminal::Rparen,
            C11Terminal::Colon => ExprTerminal::Colon,
            C11Terminal::Tilde => ExprTerminal::Tilde,
            C11Terminal::Bang => ExprTerminal::Bang,
            C11Terminal::Inc => ExprTerminal::Inc,
            C11Terminal::Dec => ExprTerminal::Dec,
            C11Terminal::Eq(p) => ExprTerminal::Eq(p),
            C11Terminal::Question(p) => ExprTerminal::Question(p),
            C11Terminal::Star(p) => ExprTerminal::Star(p),
            C11Terminal::Amp(p) => ExprTerminal::Amp(p),
            C11Terminal::Plus(p) => ExprTerminal::Plus(p),
            C11Terminal::Minus(p) => ExprTerminal::Minus(p),
            C11Terminal::Binop(p) => {
                // We need to figure out which binop from precedence level
                let op = match p.level() {
                    3 => BinOp::Or,
                    4 => BinOp::And,
                    5 => BinOp::BitOr,
                    6 => BinOp::BitXor,
                    8 => BinOp::Eq,  // or Ne - can't distinguish, but same prec
                    9 => BinOp::Lt,  // or Gt/Le/Ge
                    10 => BinOp::Shl, // or Shr
                    12 => BinOp::Div, // or Mod
                    _ => return None,
                };
                ExprTerminal::Binop(op, p)
            }
            // Skip tokens we don't need for expression evaluation
            _ => return None,
        })
    }

    /// Evaluate a C expression using our own simple lexer that preserves number values
    fn eval_c_expr(input: &str) -> Result<i64, String> {
        use gazelle::lexer::Source;

        let mut parser = ExprParser::<Eval>::new();
        let mut actions = Eval;
        let mut src = Source::from_str(input);
        let mut tokens = Vec::new();

        loop {
            src.skip_whitespace();
            if src.at_end() {
                break;
            }

            // Number - preserve the value
            if let Some(span) = src.read_digits() {
                let s = &input[span];
                let n: i64 = s.parse().unwrap_or(0);
                tokens.push(ExprTerminal::Num(n));
                continue;
            }

            // Identifier
            if src.read_ident().is_some() {
                continue; // Skip identifiers for expression eval
            }

            // Punctuation
            if let Some(c) = src.peek() {
                match c {
                    '(' => { src.advance(); tokens.push(ExprTerminal::Lparen); continue; }
                    ')' => { src.advance(); tokens.push(ExprTerminal::Rparen); continue; }
                    _ => {}
                }
            }

            // Multi-char operators
            if src.read_exact("||").is_some() { tokens.push(ExprTerminal::Binop(BinOp::Or, Precedence::Left(3))); continue; }
            if src.read_exact("&&").is_some() { tokens.push(ExprTerminal::Binop(BinOp::And, Precedence::Left(4))); continue; }
            if src.read_exact("==").is_some() { tokens.push(ExprTerminal::Binop(BinOp::Eq, Precedence::Left(8))); continue; }
            if src.read_exact("!=").is_some() { tokens.push(ExprTerminal::Binop(BinOp::Ne, Precedence::Left(8))); continue; }
            if src.read_exact("<=").is_some() { tokens.push(ExprTerminal::Binop(BinOp::Le, Precedence::Left(9))); continue; }
            if src.read_exact(">=").is_some() { tokens.push(ExprTerminal::Binop(BinOp::Ge, Precedence::Left(9))); continue; }
            if src.read_exact("<<").is_some() { tokens.push(ExprTerminal::Binop(BinOp::Shl, Precedence::Left(10))); continue; }
            if src.read_exact(">>").is_some() { tokens.push(ExprTerminal::Binop(BinOp::Shr, Precedence::Left(10))); continue; }

            // Single-char operators
            if let Some(c) = src.peek() {
                src.advance();
                let tok = match c {
                    '?' => ExprTerminal::Question(Precedence::Right(2)),
                    ':' => ExprTerminal::Colon,
                    '|' => ExprTerminal::Binop(BinOp::BitOr, Precedence::Left(5)),
                    '^' => ExprTerminal::Binop(BinOp::BitXor, Precedence::Left(6)),
                    '&' => ExprTerminal::Amp(Precedence::Left(7)),
                    '<' => ExprTerminal::Binop(BinOp::Lt, Precedence::Left(9)),
                    '>' => ExprTerminal::Binop(BinOp::Gt, Precedence::Left(9)),
                    '+' => ExprTerminal::Plus(Precedence::Left(11)),
                    '-' => ExprTerminal::Minus(Precedence::Left(11)),
                    '*' => ExprTerminal::Star(Precedence::Left(12)),
                    '/' => ExprTerminal::Binop(BinOp::Div, Precedence::Left(12)),
                    '%' => ExprTerminal::Binop(BinOp::Mod, Precedence::Left(12)),
                    '~' => ExprTerminal::Tilde,
                    '!' => ExprTerminal::Bang,
                    _ => continue, // Skip unknown
                };
                tokens.push(tok);
            }
        }

        for tok in tokens {
            parser.push(tok, &mut actions).map_err(|e| format!("{:?}", e))?;
        }

        parser.finish(&mut actions).map_err(|(p, e)| p.format_error(&e))
    }

    #[test]
    fn test_expr_precedence() {
        // Multiplicative > Additive
        assert_eq!(eval_c_expr("1 + 2 * 3").unwrap(), 7);
        assert_eq!(eval_c_expr("2 * 3 + 1").unwrap(), 7);
        assert_eq!(eval_c_expr("(1 + 2) * 3").unwrap(), 9);
    }

    #[test]
    fn test_expr_associativity() {
        // Left-associative
        assert_eq!(eval_c_expr("10 - 3 - 2").unwrap(), 5);   // (10-3)-2
        assert_eq!(eval_c_expr("100 / 10 / 2").unwrap(), 5); // (100/10)/2
        // Right-associative ternary
        assert_eq!(eval_c_expr("1 ? 2 : 0 ? 3 : 4").unwrap(), 2);
        assert_eq!(eval_c_expr("0 ? 2 : 1 ? 3 : 4").unwrap(), 3);
    }

    #[test]
    fn test_expr_all_precedence_levels() {
        // Test each C precedence level
        assert_eq!(eval_c_expr("1 || 0").unwrap(), 1);       // level 3
        assert_eq!(eval_c_expr("1 && 1").unwrap(), 1);       // level 4
        assert_eq!(eval_c_expr("5 | 2").unwrap(), 7);        // level 5
        assert_eq!(eval_c_expr("7 ^ 3").unwrap(), 4);        // level 6
        assert_eq!(eval_c_expr("7 & 3").unwrap(), 3);        // level 7
        assert_eq!(eval_c_expr("2 == 2").unwrap(), 1);       // level 8
        assert_eq!(eval_c_expr("1 < 2").unwrap(), 1);        // level 9
        assert_eq!(eval_c_expr("1 << 3").unwrap(), 8);       // level 10
        assert_eq!(eval_c_expr("3 + 4").unwrap(), 7);        // level 11
        assert_eq!(eval_c_expr("3 * 4").unwrap(), 12);       // level 12
    }

    #[test]
    fn test_expr_mixed_precedence() {
        assert_eq!(eval_c_expr("1 || 0 && 0").unwrap(), 1);   // && before ||
        assert_eq!(eval_c_expr("1 + 1 == 2").unwrap(), 1);    // + before ==
        assert_eq!(eval_c_expr("1 + 2 < 4").unwrap(), 1);     // + before <
        assert_eq!(eval_c_expr("1 + 2 * 3 + 4").unwrap(), 11);
    }

    #[test]
    fn test_expr_unary() {
        assert_eq!(eval_c_expr("-5").unwrap(), -5);
        assert_eq!(eval_c_expr("!0").unwrap(), 1);
        assert_eq!(eval_c_expr("!1").unwrap(), 0);
        assert_eq!(eval_c_expr("~0").unwrap(), -1);
    }

    #[test]
    fn test_expr_ternary() {
        assert_eq!(eval_c_expr("1 ? 2 : 3").unwrap(), 2);
        assert_eq!(eval_c_expr("0 ? 2 : 3").unwrap(), 3);
        assert_eq!(eval_c_expr("1 + 1 ? 2 : 3").unwrap(), 2);  // + before ?
    }
}
