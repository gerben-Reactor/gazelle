use gazelle_macros::gazelle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AugOp {
    Add, Sub, Mul, Div, FloorDiv, Mod, Pow, Shl, Shr, BitAnd, BitOr, BitXor, MatMul,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompOp {
    Eq, Ne, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Div, FloorDiv, Mod, Shl, Shr, BitAnd, BitOr, BitXor,
}

gazelle! {
    pub(crate) grammar Python {
        start file_input;
        terminals {
            NAME: Name, NUMBER: Name, STRING: Name,
            // Keywords
            FALSE, NONE, TRUE,
            AND, AS, ASSERT, ASYNC, AWAIT,
            BREAK, CLASS, CONTINUE, DEF, DEL,
            ELIF, ELSE, EXCEPT, FINALLY,
            FOR, FROM, GLOBAL, IF, IMPORT, IN, IS,
            LAMBDA, NONLOCAL, NOT, OR,
            PASS, RAISE, RETURN,
            TRY, WHILE, WITH, YIELD,
            // Delimiters
            LPAREN, RPAREN, LBRACK, RBRACK, LBRACE, RBRACE,
            COLON, SEMICOLON, COMMA, DOT, ELLIPSIS,
            ARROW, AT, WALRUS,
            EQ,
            AUGASSIGN: AugOp,
            NEWLINE, INDENT, DEDENT,
            // Comparison operators (not precedence-based)
            COMP_OP: CompOp,
            // Unary
            TILDE,
            // Precedence-based operators
            prec PLUS, prec MINUS,
            prec STAR,
            prec DOUBLESTAR,
            prec BINOP: BinOp
        }

        // ================================================================
        // File structure
        // ================================================================

        file_input = statements? => file_input;
        statements = statement+ => statements;
        statement = compound_stmt => compound_stmt | simple_stmts => simple_stmts;
        simple_stmts = (simple_stmt % SEMICOLON) SEMICOLON? NEWLINE => simple_stmts;

        // ================================================================
        // Simple statements
        // ================================================================

        simple_stmt = star_expressions => expr
                    | star_expressions EQ assign_rhs => assign
                    | star_expressions AUGASSIGN star_expressions => aug_assign
                    | star_expressions COLON expression => annotate
                    | star_expressions COLON expression EQ star_expressions => annotate_assign
                    | RETURN star_expressions? => return
                    | IMPORT dotted_as_names => import
                    | FROM import_from_path IMPORT import_targets => from_import
                    | RAISE raise_args? => raise
                    | PASS => pass
                    | BREAK => break
                    | CONTINUE => continue
                    | DEL star_expressions => del
                    | YIELD yield_arg? => yield
                    | ASSERT expression assert_msg? => assert
                    | GLOBAL (NAME % COMMA) => global
                    | NONLOCAL (NAME % COMMA) => nonlocal;

        assert_msg = COMMA expression => assert_msg;

        assign_rhs = star_expressions => star_expressions
                   | yield_expr => yield_expr
                   | star_expressions EQ assign_rhs => chain;

        yield_expr = YIELD yield_arg? => yield_expr;
        yield_arg = FROM expression => from | star_expressions => star_expressions;

        raise_args = expression raise_from? => raise_args;
        raise_from = FROM expression => raise_from;

        // Import paths
        dotted_name = (NAME % DOT) => dotted_name;
        dotted_as_name = dotted_name as_name? => dotted_as_name;
        dotted_as_names = (dotted_as_name % COMMA) => dotted_as_names;
        as_name = AS NAME => as_name;

        import_from_path = dots? dotted_name => dotted | dots => dots;
        dots = DOT+ => dots;

        import_targets = STAR => star
                       | (import_as_name % COMMA) COMMA? => names
                       | LPAREN (import_as_name % COMMA) COMMA? RPAREN => paren_names;

        import_as_name = NAME as_name? => import_as_name;

        // ================================================================
        // Expressions
        // ================================================================

        // Arithmetic with runtime precedence
        arith_expr = primary => primary
                   | arith_expr BINOP arith_expr => binop
                   | arith_expr STAR arith_expr => mul
                   | arith_expr PLUS arith_expr => add
                   | arith_expr MINUS arith_expr => sub
                   | arith_expr DOUBLESTAR arith_expr => pow
                   | PLUS arith_expr => pos | MINUS arith_expr => neg | TILDE arith_expr => invert
                   | AWAIT arith_expr => await;

        // Star targets — for LHS of for-loops (no comparison, avoids IN ambiguity)
        star_target = STAR arith_expr => star | arith_expr => arith_expr;
        star_targets = (star_target % COMMA) COMMA? => star_targets;

        // Star expressions — listed before comparison so star_expression wins
        // RR conflicts with comparison (LALR inadequacy, not a true ambiguity)
        star_expression = STAR arith_expr => star | expression => expression;
        star_expressions = (star_expression % COMMA) COMMA? => star_expressions;
        star_named_expression = STAR arith_expr => star | named_expression => named_expression;

        // Comparison: flat chain
        comparison = arith_expr => arith_expr
                   | arith_expr comp_pair+ => compare;

        comp_pair = COMP_OP arith_expr => comp_op
                  | IN arith_expr => in
                  | NOT IN arith_expr => not_in
                  | IS arith_expr => is
                  | IS NOT arith_expr => is_not;

        // Logical
        inversion = comparison => comparison | NOT inversion => not;
        conjunction = inversion => inversion | conjunction AND inversion => and;
        disjunction = conjunction => conjunction | disjunction OR conjunction => or;

        // Named expression (walrus)
        named_expression = disjunction => disjunction | NAME WALRUS disjunction => walrus;

        // Full expression with ternary and lambda
        expression = disjunction => disjunction
                   | disjunction IF disjunction ELSE expression => ternary
                   | lambda_expr => lambda_expr;

        lambda_expr = LAMBDA lambda_params? COLON expression => lambda_expr;

        lambda_params = (lambda_param % COMMA) COMMA? => lambda_params;
        lambda_param = NAME lambda_default? => name
                     | STAR NAME? => star
                     | DOUBLESTAR NAME => double_star;
        lambda_default = EQ expression => lambda_default;

        // ================================================================
        // Primary / Atom
        // ================================================================

        primary = atom => atom
                | primary DOT NAME => attr
                | primary LPAREN arguments? RPAREN => call
                | primary LBRACK slices RBRACK => subscript;

        atom = NAME => name
             | NUMBER => number
             | string_concat => string_concat
             | TRUE => true | FALSE => false | NONE => none | ELLIPSIS => ellipsis
             | LPAREN paren_body? RPAREN => paren
             | LBRACK list_body? RBRACK => list
             | LBRACE dict_or_set? RBRACE => dict_or_set;

        string_concat = STRING+ => string_concat;

        // Inside parens: tuple, generator expr, or yield
        paren_body = star_named_expression comp_for => generator
                   | star_expressions => star_expressions
                   | yield_expr => yield_expr;

        // Inside brackets: list or list comp
        list_body = star_named_expression comp_for => list_comp
                  | star_expressions => star_expressions;

        // ================================================================
        // Slices
        // ================================================================

        slices = (slice % COMMA) COMMA? => slices;
        slice = expression => expression
              | expression? COLON expression? slice_step? => slice;

        slice_step = COLON expression? => slice_step;

        // ================================================================
        // Arguments
        // ================================================================

        arguments = (arg % COMMA) COMMA? => arguments;
        arg = star_expression comp_for => generator
            | star_expression => star_expression
            | NAME EQ expression => keyword
            | DOUBLESTAR expression => double_star;

        // ================================================================
        // Dict/Set
        // ================================================================

        dict_or_set = dict_comp => dict_comp | set_comp => set_comp | dict_items => dict_items | star_expressions => set;

        dict_items = (kvpair % COMMA) COMMA? => dict_items;
        kvpair = expression COLON expression => kvpair | DOUBLESTAR expression => double_star;

        dict_comp = expression COLON expression comp_for => dict_comp;
        set_comp = star_named_expression comp_for => set_comp;

        // ================================================================
        // Comprehensions
        // ================================================================

        comp_for = FOR star_targets IN disjunction filter* comp_for? => comp_for;
        filter = IF disjunction => filter;

        // ================================================================
        // Compound statements
        // ================================================================

        compound_stmt = if_stmt => if_stmt | while_stmt => while_stmt | for_stmt => for_stmt | try_stmt => try_stmt
                      | with_stmt => with_stmt | func_def => func_def | class_def => class_def | async_stmt => async_stmt;

        block = NEWLINE INDENT statements DEDENT => block;

        if_stmt = IF named_expression COLON block elif_clause* else_clause? => if_stmt;
        elif_clause = ELIF named_expression COLON block => elif_clause;
        else_clause = ELSE COLON block => else_clause;

        while_stmt = WHILE named_expression COLON block else_clause? => while_stmt;

        for_stmt = FOR star_targets IN star_expressions COLON block else_clause? => for_stmt;

        try_stmt = TRY COLON block except_clause+ else_clause? finally_clause? => try_except
                 | TRY COLON block finally_clause => try_finally;

        except_clause = EXCEPT expression except_as? COLON block => except
                      | EXCEPT COLON block => except_all;
        except_as = AS NAME => except_as;

        finally_clause = FINALLY COLON block => finally_clause;

        with_stmt = WITH (with_item % COMMA) COLON block => with_stmt;
        with_item = expression with_as? => with_item;
        with_as = AS star_expression => with_as;

        func_def = decorators? DEF NAME LPAREN params? RPAREN return_annot? COLON block => func_def;
        return_annot = ARROW expression => return_annot;
        decorators = decorator+ => decorators;
        decorator = AT expression NEWLINE => decorator;

        params = (param % COMMA) COMMA? => params;
        param = NAME param_annot? param_default? => name
              | STAR NAME? param_annot? => star
              | DOUBLESTAR NAME param_annot? => double_star;
        param_annot = COLON expression => param_annot;
        param_default = EQ expression => param_default;

        class_def = decorators? CLASS NAME class_args? COLON block => class_def;
        class_args = LPAREN arguments? RPAREN => class_args;

        async_stmt = ASYNC func_def => async_func
                   | ASYNC for_stmt => async_for
                   | ASYNC with_stmt => async_with;
    }
}

// Dummy actions — all types are ()
pub struct PyActions;

impl PythonTypes for PyActions {
    type Error = gazelle::ParseError;
    type Name = String;
    type AugOp = AugOp;
    type CompOp = CompOp;
    type BinOp = BinOp;
    type File_input = gazelle::Ignore;
    type Statements = gazelle::Ignore;
    type Statement = gazelle::Ignore;
    type Simple_stmts = gazelle::Ignore;
    type Simple_stmt = gazelle::Ignore;
    type Assert_msg = gazelle::Ignore;
    type Assign_rhs = gazelle::Ignore;
    type Yield_expr = gazelle::Ignore;
    type Yield_arg = gazelle::Ignore;
    type Raise_args = gazelle::Ignore;
    type Raise_from = gazelle::Ignore;
    type Dotted_name = gazelle::Ignore;
    type Dotted_as_name = gazelle::Ignore;
    type Dotted_as_names = gazelle::Ignore;
    type As_name = gazelle::Ignore;
    type Import_from_path = gazelle::Ignore;
    type Dots = gazelle::Ignore;
    type Import_targets = gazelle::Ignore;
    type Import_as_name = gazelle::Ignore;
    type Arith_expr = gazelle::Ignore;
    type Star_target = gazelle::Ignore;
    type Star_targets = gazelle::Ignore;
    type Star_expression = gazelle::Ignore;
    type Star_expressions = gazelle::Ignore;
    type Star_named_expression = gazelle::Ignore;
    type Comparison = gazelle::Ignore;
    type Comp_pair = gazelle::Ignore;
    type Inversion = gazelle::Ignore;
    type Conjunction = gazelle::Ignore;
    type Disjunction = gazelle::Ignore;
    type Named_expression = gazelle::Ignore;
    type Expression = gazelle::Ignore;
    type Lambda_expr = gazelle::Ignore;
    type Lambda_params = gazelle::Ignore;
    type Lambda_param = gazelle::Ignore;
    type Lambda_default = gazelle::Ignore;
    type Primary = gazelle::Ignore;
    type Atom = gazelle::Ignore;
    type String_concat = gazelle::Ignore;
    type Paren_body = gazelle::Ignore;
    type List_body = gazelle::Ignore;
    type Slices = gazelle::Ignore;
    type Slice = gazelle::Ignore;
    type Slice_step = gazelle::Ignore;
    type Arguments = gazelle::Ignore;
    type Arg = gazelle::Ignore;
    type Dict_or_set = gazelle::Ignore;
    type Dict_items = gazelle::Ignore;
    type Kvpair = gazelle::Ignore;
    type Dict_comp = gazelle::Ignore;
    type Set_comp = gazelle::Ignore;
    type Comp_for = gazelle::Ignore;
    type Filter = gazelle::Ignore;
    type Compound_stmt = gazelle::Ignore;
    type Block = gazelle::Ignore;
    type If_stmt = gazelle::Ignore;
    type Elif_clause = gazelle::Ignore;
    type Else_clause = gazelle::Ignore;
    type While_stmt = gazelle::Ignore;
    type For_stmt = gazelle::Ignore;
    type Try_stmt = gazelle::Ignore;
    type Except_clause = gazelle::Ignore;
    type Except_as = gazelle::Ignore;
    type Finally_clause = gazelle::Ignore;
    type With_stmt = gazelle::Ignore;
    type With_item = gazelle::Ignore;
    type With_as = gazelle::Ignore;
    type Func_def = gazelle::Ignore;
    type Return_annot = gazelle::Ignore;
    type Decorators = gazelle::Ignore;
    type Decorator = gazelle::Ignore;
    type Params = gazelle::Ignore;
    type Param = gazelle::Ignore;
    type Param_annot = gazelle::Ignore;
    type Param_default = gazelle::Ignore;
    type Class_def = gazelle::Ignore;
    type Class_args = gazelle::Ignore;
    type Async_stmt = gazelle::Ignore;
}

// PythonActions is auto-implemented via blanket impl

