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

        file_input = statements?;
        statements = statement+;
        statement = compound_stmt | simple_stmts;
        simple_stmts = (simple_stmt % SEMICOLON) SEMICOLON? NEWLINE;

        // ================================================================
        // Simple statements
        // ================================================================

        simple_stmt = star_expressions
                    | star_expressions EQ assign_rhs
                    | star_expressions AUGASSIGN star_expressions
                    | star_expressions COLON expression
                    | star_expressions COLON expression EQ star_expressions
                    | RETURN star_expressions?
                    | IMPORT dotted_as_names
                    | FROM import_from_path IMPORT import_targets
                    | RAISE raise_args?
                    | PASS
                    | BREAK
                    | CONTINUE
                    | DEL star_expressions
                    | YIELD yield_arg?
                    | ASSERT expression assert_msg?
                    | GLOBAL (NAME % COMMA)
                    | NONLOCAL (NAME % COMMA);

        assert_msg = COMMA expression;

        assign_rhs = star_expressions
                   | yield_expr
                   | star_expressions EQ assign_rhs;

        yield_expr = YIELD yield_arg?;
        yield_arg = FROM expression | star_expressions;

        raise_args = expression raise_from?;
        raise_from = FROM expression;

        // Import paths
        dotted_name = (NAME % DOT);
        dotted_as_name = dotted_name as_name?;
        dotted_as_names = (dotted_as_name % COMMA);
        as_name = AS NAME;

        import_from_path = dots? dotted_name | dots;
        dots = DOT+;

        import_targets = STAR
                       | (import_as_name % COMMA) COMMA?
                       | LPAREN (import_as_name % COMMA) COMMA? RPAREN;

        import_as_name = NAME as_name?;

        // ================================================================
        // Expressions
        // ================================================================

        // Arithmetic with runtime precedence
        arith_expr = primary
                   | arith_expr BINOP arith_expr
                   | arith_expr STAR arith_expr
                   | arith_expr PLUS arith_expr
                   | arith_expr MINUS arith_expr
                   | arith_expr DOUBLESTAR arith_expr
                   | PLUS arith_expr | MINUS arith_expr | TILDE arith_expr
                   | AWAIT arith_expr;

        // Star targets — for LHS of for-loops (no comparison, avoids IN ambiguity)
        star_target = STAR arith_expr | arith_expr;
        star_targets = (star_target % COMMA) COMMA?;

        // Star expressions — listed before comparison so star_expression wins
        // RR conflicts with comparison (LALR inadequacy, not a true ambiguity)
        star_expression = STAR arith_expr | expression;
        star_expressions = (star_expression % COMMA) COMMA?;
        star_named_expression = STAR arith_expr | named_expression;

        // Comparison: flat chain
        comparison = arith_expr
                   | arith_expr comp_pair+;

        comp_pair = COMP_OP arith_expr
                  | IN arith_expr
                  | NOT IN arith_expr
                  | IS arith_expr
                  | IS NOT arith_expr;

        // Logical
        inversion = comparison | NOT inversion;
        conjunction = inversion | conjunction AND inversion;
        disjunction = conjunction | disjunction OR conjunction;

        // Named expression (walrus)
        named_expression = disjunction | NAME WALRUS disjunction;

        // Full expression with ternary and lambda
        expression = disjunction
                   | disjunction IF disjunction ELSE expression
                   | lambda_expr;

        lambda_expr = LAMBDA lambda_params? COLON expression;

        lambda_params = (lambda_param % COMMA) COMMA?;
        lambda_param = NAME lambda_default?
                     | STAR NAME?
                     | DOUBLESTAR NAME;
        lambda_default = EQ expression;

        // ================================================================
        // Primary / Atom
        // ================================================================

        primary = atom
                | primary DOT NAME
                | primary LPAREN arguments? RPAREN
                | primary LBRACK slices RBRACK;

        atom = NAME
             | NUMBER
             | string_concat
             | TRUE | FALSE | NONE | ELLIPSIS
             | LPAREN paren_body? RPAREN
             | LBRACK list_body? RBRACK
             | LBRACE dict_or_set? RBRACE;

        string_concat = STRING+;

        // Inside parens: tuple, generator expr, or yield
        paren_body = star_named_expression comp_for
                   | star_expressions
                   | yield_expr;

        // Inside brackets: list or list comp
        list_body = star_named_expression comp_for
                  | star_expressions;

        // ================================================================
        // Slices
        // ================================================================

        slices = (slice % COMMA) COMMA?;
        slice = expression
              | expression? COLON expression? slice_step?;

        slice_step = COLON expression?;

        // ================================================================
        // Arguments
        // ================================================================

        arguments = (arg % COMMA) COMMA?;
        arg = star_expression comp_for
            | star_expression
            | NAME EQ expression
            | DOUBLESTAR expression;

        // ================================================================
        // Dict/Set
        // ================================================================

        dict_or_set = dict_comp | set_comp | dict_items | star_expressions;

        dict_items = (kvpair % COMMA) COMMA?;
        kvpair = expression COLON expression | DOUBLESTAR expression;

        dict_comp = expression COLON expression comp_for;
        set_comp = star_named_expression comp_for;

        // ================================================================
        // Comprehensions
        // ================================================================

        comp_for = FOR star_targets IN disjunction filter* comp_for?;
        filter = IF disjunction;

        // ================================================================
        // Compound statements
        // ================================================================

        compound_stmt = if_stmt | while_stmt | for_stmt | try_stmt
                      | with_stmt | func_def | class_def | async_stmt;

        block = NEWLINE INDENT statements DEDENT;

        if_stmt = IF named_expression COLON block elif_clause* else_clause?;
        elif_clause = ELIF named_expression COLON block;
        else_clause = ELSE COLON block;

        while_stmt = WHILE named_expression COLON block else_clause?;

        for_stmt = FOR star_targets IN star_expressions COLON block else_clause?;

        try_stmt = TRY COLON block except_clause+ else_clause? finally_clause?
                 | TRY COLON block finally_clause;

        except_clause = EXCEPT expression except_as? COLON block
                      | EXCEPT COLON block;
        except_as = AS NAME;

        finally_clause = FINALLY COLON block;

        with_stmt = WITH (with_item % COMMA) COLON block;
        with_item = expression with_as?;
        with_as = AS star_expression;

        func_def = decorators? DEF NAME LPAREN params? RPAREN return_annot? COLON block;
        return_annot = ARROW expression;
        decorators = decorator+;
        decorator = AT expression NEWLINE;

        params = (param % COMMA) COMMA?;
        param = NAME param_annot? param_default?
              | STAR NAME? param_annot?
              | DOUBLESTAR NAME param_annot?;
        param_annot = COLON expression;
        param_default = EQ expression;

        class_def = decorators? CLASS NAME class_args? COLON block;
        class_args = LPAREN arguments? RPAREN;

        async_stmt = ASYNC func_def
                   | ASYNC for_stmt
                   | ASYNC with_stmt;
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
}

// PythonActions is auto-implemented via blanket impl

