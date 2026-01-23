//! Reduction analysis for trait-based code generation.

use super::CodegenContext;

/// Information about how to handle a reduction.
#[derive(Debug, Clone)]
pub enum ReductionKind {
    /// Named reduction - call trait method with name.
    Named {
        method_name: String,
        /// Typed parameters: (symbol_index, type_string).
        params: Vec<(usize, String)>,
    },
    /// Passthrough - single typed symbol, return it directly.
    Passthrough {
        /// Index of the typed symbol in RHS.
        symbol_index: usize,
    },
    /// Structural - no typed symbols, no user code needed.
    Structural,
}

/// Information about a reduction for code generation.
#[derive(Debug, Clone)]
pub struct ReductionInfo {
    /// The non-terminal name (LHS).
    pub non_terminal: String,
    /// How to handle this reduction.
    pub kind: ReductionKind,
    /// All RHS symbols with their types (for stack manipulation).
    pub rhs_symbols: Vec<SymbolInfo>,
}

/// Information about a symbol in a reduction RHS.
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub ty: Option<String>,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolKind {
    UnitTerminal,
    PayloadTerminal,
    PrecTerminal,
    NonTerminal,
}

/// Analyze all rules and build reduction info.
pub fn analyze_reductions(ctx: &CodegenContext) -> Result<Vec<ReductionInfo>, String> {
    let mut result = Vec::new();

    for rule_info in &ctx.rules {
        let result_type = rule_info.result_type.clone();

        for alt in &rule_info.alternatives {
            // Build symbol info for this alternative
            let mut rhs_symbols = Vec::new();
            for (sym_name, sym_type) in &alt.symbols {
                let kind = determine_symbol_kind(ctx, sym_name);
                rhs_symbols.push(SymbolInfo {
                    name: sym_name.clone(),
                    ty: sym_type.clone(),
                    kind,
                });
            }

            // Count typed symbols
            let typed_symbols: Vec<(usize, &String)> = rhs_symbols.iter()
                .enumerate()
                .filter_map(|(i, s)| s.ty.as_ref().map(|t| (i, t)))
                .collect();

            // Determine reduction kind
            let kind = if let Some(name) = &alt.name {
                // Named reduction -> trait method
                let params: Vec<_> = typed_symbols.iter()
                    .map(|(i, t)| (*i, (*t).clone()))
                    .collect();
                ReductionKind::Named {
                    method_name: name.clone(),
                    params,
                }
            } else if result_type.is_none() {
                // No result type -> structural
                ReductionKind::Structural
            } else if typed_symbols.len() == 1 {
                // Single typed symbol - check if it's the same non-terminal for passthrough
                let (idx, _) = typed_symbols[0];
                let sym = &rhs_symbols[idx];

                if sym.kind == SymbolKind::NonTerminal && sym.name == rule_info.name {
                    // Same non-terminal to same non-terminal passthrough
                    // (e.g., expr: ... = LPAREN expr RPAREN; returns inner expr)
                    ReductionKind::Passthrough { symbol_index: idx }
                } else if sym.kind == SymbolKind::NonTerminal {
                    // Different non-terminals have different associated types
                    // Require @name to convert between them
                    return Err(format!(
                        "Rule '{}' alternative has single non-terminal '{}' (different type). \
                         Use @name to convert '{}' to '{}'.",
                        rule_info.name,
                        sym.name,
                        sym.name,
                        rule_info.name
                    ));
                } else {
                    // Terminal to non-terminal: can't passthrough with associated types
                    // because we don't know if i32 == A::Value at codegen time.
                    return Err(format!(
                        "Rule '{}' alternative has single typed terminal '{}'. \
                         With trait-based API, use @name to convert terminal value to result type.",
                        rule_info.name,
                        sym.name
                    ));
                }
            } else if typed_symbols.is_empty() {
                if result_type.is_some() {
                    // Has result type but no typed symbols - error
                    return Err(format!(
                        "Rule '{}' alternative has result type but no typed symbols. \
                         Add @name to specify how to create the result, or add a typed symbol.",
                        rule_info.name
                    ));
                }
                ReductionKind::Structural
            } else {
                // Multiple typed symbols without name -> error
                return Err(format!(
                    "Rule '{}' has alternative with {} typed symbols but no @name. \
                     Add @name to specify how to combine them, or reduce to a single typed symbol.",
                    rule_info.name,
                    typed_symbols.len()
                ));
            };

            result.push(ReductionInfo {
                non_terminal: rule_info.name.clone(),
                kind,
                rhs_symbols,
            });
        }
    }

    Ok(result)
}

fn determine_symbol_kind(ctx: &CodegenContext, name: &str) -> SymbolKind {
    // Check if it's a regular terminal
    for (id, _) in &ctx.terminal_types {
        if let Some(sym_name) = ctx.symbol_names.get(id) {
            if sym_name == name {
                if ctx.terminal_types.get(id).map(|t| t.is_some()).unwrap_or(false) {
                    return SymbolKind::PayloadTerminal;
                } else {
                    return SymbolKind::UnitTerminal;
                }
            }
        }
    }

    // Check if it's a prec terminal
    for (id, _) in &ctx.prec_terminal_types {
        if let Some(sym_name) = ctx.symbol_names.get(id) {
            if sym_name == name {
                return SymbolKind::PrecTerminal;
            }
        }
    }

    // Must be a non-terminal
    SymbolKind::NonTerminal
}

/// Collect unique trait methods from all reductions.
pub fn collect_trait_methods(reductions: &[ReductionInfo]) -> Vec<TraitMethod> {
    let mut methods = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for info in reductions {
        if let ReductionKind::Named { method_name, params } = &info.kind {
            // Use non_terminal + method_name as key to handle potential conflicts
            let key = format!("{}_{}", info.non_terminal, method_name);
            if seen.insert(key) {
                methods.push(TraitMethod {
                    name: method_name.clone(),
                    non_terminal: info.non_terminal.clone(),
                    params: params.clone(),
                    rhs_symbols: info.rhs_symbols.clone(),
                });
            }
        }
    }

    methods
}

/// Information about a trait method to generate.
#[derive(Debug, Clone)]
pub struct TraitMethod {
    pub name: String,
    pub non_terminal: String,
    /// (symbol_index, type_string) for each parameter.
    pub params: Vec<(usize, String)>,
    /// Full RHS symbols (for documentation).
    pub rhs_symbols: Vec<SymbolInfo>,
}
