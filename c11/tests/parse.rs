use std::path::Path;

/// C11parser test suite (Jourdan's typedef disambiguation tests).
/// These don't use #include so raw parse is fine.
const C11_TEST_FILES: &[&str] = &[
    "typedef_star.c",
    "variable_star.c",
    "local_typedef.c",
    "block_scope.c",
    "declaration_ambiguity.c",
    "enum.c",
    "enum_shadows_typedef.c",
    "enum_constant_visibility.c",
    "namespaces.c",
    "local_scope.c",
    "if_scopes.c",
    "loop_scopes.c",
    "no_local_scope.c",
    "function_parameter_scope.c",
    "function_parameter_scope_extends.c",
    "argument_scope.c",
    "control-scope.c",
    "dangling_else.c",
    "dangling_else_lookahead.c",
    "dangling_else_lookahead.if.c",
    "parameter_declaration_ambiguity.c",
    "parameter_declaration_ambiguity.test.c",
    "bitfield_declaration_ambiguity.c",
    "bitfield_declaration_ambiguity.ok.c",
    "expressions.c",
    "statements.c",
    "types.c",
    "declarators.c",
    "designator.c",
    "function-decls.c",
    "struct-recursion.c",
    "long-long-struct.c",
    "c-namespace.c",
    "enum-trick.c",
    "char-literal-printing.c",
    "c11-noreturn.c",
    "c1x-alignas.c",
    "atomic.c",
    "atomic_parenthesis.c",
    "aligned_struct_c18.c",
    "declarator_visibility.c",
];

#[test]
fn test_c11parser_suite() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../examples/c11/C11parser/tests");

    let mut passed = 0;
    let mut failed = Vec::new();

    for name in C11_TEST_FILES {
        let path = dir.join(name);
        let content = std::fs::read_to_string(&path).unwrap();
        match c11::parse(&content) {
            Ok(_) => passed += 1,
            Err(e) => {
                eprintln!("FAIL: {}: {}", name, e);
                failed.push(e);
            }
        }
    }

    eprintln!("{} passed, {} failed", passed, failed.len());
    if !failed.is_empty() {
        panic!("{} tests failed", failed.len());
    }
}

/// GCC extensions our grammar doesn't support.
const EXPECTED_FAILURES: &[&str] = &[
    "00213.c", // GCC statement expressions ({...})
    "00214.c", // GCC statement expressions ({...})
    "00216.c", // GCC empty struct
];

#[test]
fn test_c_testsuite() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/c-testsuite/tests/single-exec");

    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("Cannot read {}: {}", dir.display(), e))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "c"))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut passed = 0;
    let mut unexpected_failures = Vec::new();
    let mut unexpected_passes = Vec::new();

    for entry in &entries {
        let name = entry.file_name();
        let name = name.to_str().unwrap();
        let expected_fail = EXPECTED_FAILURES.contains(&name);
        match c11::parse_file(&entry.path()) {
            Ok(_) => {
                passed += 1;
                if expected_fail {
                    unexpected_passes.push(name.to_string());
                }
            }
            Err(e) => {
                if !expected_fail {
                    unexpected_failures.push(format!("{}: {}", name, e));
                }
            }
        }
    }

    eprintln!("{} passed, {} expected failures, {} c-testsuite files",
        passed, EXPECTED_FAILURES.len(), entries.len());

    for e in &unexpected_failures {
        eprintln!("UNEXPECTED FAIL: {}", e);
    }
    for p in &unexpected_passes {
        eprintln!("UNEXPECTED PASS (remove from EXPECTED_FAILURES): {}", p);
    }

    assert!(unexpected_failures.is_empty(), "{} unexpected failures", unexpected_failures.len());
    assert!(unexpected_passes.is_empty(), "{} unexpected passes", unexpected_passes.len());
}
