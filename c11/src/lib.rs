pub mod ast;
pub mod codegen;
pub mod grammar;
pub mod lexer;
pub mod typecheck;
pub mod types;

use std::path::Path;
use std::process::Command;

use grammar::{C11Parser, CActions};
use lexer::C11Lexer;

/// Run `cc -E` on a file, returning the preprocessed source.
pub fn preprocess(path: &Path) -> Result<String, String> {
    let output = Command::new("cc")
        .args(["-E", "-std=c11", "-xc"])
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run cc -E: {}", e))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Parse preprocessed C11 source (strips `#` line markers from `cc -E` output).
pub fn parse(input: &str) -> Result<ast::TranslationUnit, String> {
    let stripped: String = input
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    let mut parser = C11Parser::<CActions>::new();
    let mut actions = CActions::new();
    let mut lexer = C11Lexer::new(&stripped);
    let mut token_count = 0;

    loop {
        match lexer.next(&mut actions)? {
            Some(t) => {
                token_count += 1;
                parser.push(t, &mut actions).map_err(|e| {
                    format!("Parse error at token {}: {:?}", token_count, e)
                })?;
            }
            None => break,
        }
    }

    parser.finish(&mut actions).map_err(|(p, e)| format!("Finish error: {}", p.format_error(&e)))?;
    Ok(actions.unit)
}

/// Preprocess and parse a C file.
pub fn parse_file(path: &Path) -> Result<ast::TranslationUnit, String> {
    let source = preprocess(path)?;
    parse(&source)
}
