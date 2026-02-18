pub mod ast;
pub mod codegen;
pub mod grammar;
pub mod lexer;
pub mod typecheck;
pub mod types;

use std::path::Path;
use std::process::Command;

use gazelle::lexer::Scanner;

use grammar::{C11, CActions};

type T = C11::Terminal<CActions>;

/// Run `cc -E` on a file, returning the preprocessed source.
pub fn preprocess(path: &Path) -> Result<String, String> {
    let output = Command::new("cc")
        .args(["-E", "-std=gnu11", "-xc"])
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run cc -E: {}", e))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Parse preprocessed C11 source.
pub fn parse(input: &str) -> Result<ast::TranslationUnit, String> {
    let mut parser = C11::Parser::<CActions>::new();
    let mut actions = CActions::new();
    let mut src = Scanner::new(input);

    loop {
        src.skip_whitespace();
        while src.skip_line_comment("//") || src.skip_block_comment("/*", "*/") {
            src.skip_whitespace();
        }
        if src.at_end() { break; }

        // Skip preprocessor line markers
        if src.peek() == Some('#') {
            src.skip_while(|c| c != '\n');
            continue;
        }

        let tok = lexer::next_token(&mut src, input)?;

        if let T::Name(name) = tok {
            match name.as_str() {
                "__attribute__" | "__attribute" | "__asm__" | "__asm" | "asm" => {
                    lexer::skip_balanced_parens(&mut src);
                    continue;
                }
                "__extension__" => continue,
                _ => {
                    parser.push(T::Name(name.clone()), &mut actions).map_err(|e| parser.format_error(&e))?;
                    let kind = if actions.ctx.is_typedef(&name) { T::Type } else { T::Variable };
                    parser.push(kind, &mut actions).map_err(|e| parser.format_error(&e))?;
                }
            }
        } else {
            parser.push(tok, &mut actions).map_err(|e| parser.format_error(&e))?;
        }
    }

    let defs = parser.finish(&mut actions).map_err(|(p, e)| p.format_error(&e))?;
    let mut unit = ast::TranslationUnit { decls: vec![], functions: vec![], structs: Default::default() };
    for def in defs {
        match def {
            C11::ExternalDeclaration::Decl(d) => unit.decls.push(d),
            C11::ExternalDeclaration::FuncDef(f) => unit.functions.push(f),
        }
    }
    Ok(unit)
}

/// Preprocess and parse a C file.
pub fn parse_file(path: &Path) -> Result<ast::TranslationUnit, String> {
    let source = preprocess(path)?;
    parse(&source)
}
