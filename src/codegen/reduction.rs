//! Reduction analysis for trait-based code generation.

use crate::lr::AltAction;
use super::CodegenContext;

/// Information about a reduction for code generation.
#[derive(Debug, Clone)]
pub struct ReductionInfo {
    /// The non-terminal name (LHS).
    pub non_terminal: String,
    /// The action for this reduction (from the grammar rule).
    pub action: AltAction,
    /// For `AltAction::None` with a typed result: the index of the single typed symbol to pass through.
    pub passthrough_index: Option<usize>,
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
#[allow(clippy::enum_variant_names)]
pub enum SymbolKind {
    UnitTerminal,
    PayloadTerminal,
    PrecTerminal,
    NonTerminal,
}

/// Analyze all rules and build reduction info.
pub fn analyze_reductions(ctx: &CodegenContext) -> Result<Vec<ReductionInfo>, String> {
    let grammar = &ctx.grammar;
    let mut result = Vec::new();

    // Skip rule 0 (__start -> original_start)
    for rule in &grammar.rules[1..] {
        let nt_name = grammar.symbols.name(rule.lhs.id()).to_string();
        let result_type = grammar.types.get(&rule.lhs.id())
            .and_then(|t| t.clone());

        // Build symbol info for this alternative
        let mut rhs_symbols = Vec::new();
        for sym in &rule.rhs {
            let sym_name = grammar.symbols.name(sym.id()).to_string();
            let kind = determine_symbol_kind(ctx, sym);
            let ty = symbol_type(ctx, sym);
            rhs_symbols.push(SymbolInfo {
                name: sym_name,
                ty,
                kind,
            });
        }

        // Count typed symbols
        let typed_symbols: Vec<(usize, &String)> = rhs_symbols.iter()
            .enumerate()
            .filter_map(|(i, s)| s.ty.as_ref().map(|t| (i, t)))
            .collect();

        // Validate AltAction::None and compute passthrough index
        let passthrough_index = if matches!(rule.action, AltAction::None) {
            if let Some(result_type) = &result_type {
                if typed_symbols.len() == 1 {
                    let (idx, sym_type) = typed_symbols[0];
                    if sym_type == result_type {
                        Some(idx)
                    } else {
                        let sym = &rhs_symbols[idx];
                        return Err(format!(
                            "Rule '{}' has type '{}' but symbol '{}' has type '{}'. \
                             Use @name to convert.",
                            nt_name, result_type, sym.name, sym_type
                        ));
                    }
                } else if typed_symbols.is_empty() {
                    return Err(format!(
                        "Rule '{}' alternative has result type but no typed symbols. \
                         Add @name to specify how to create the result.",
                        nt_name
                    ));
                } else {
                    return Err(format!(
                        "Rule '{}' has alternative with {} typed symbols but no @name.",
                        nt_name, typed_symbols.len()
                    ));
                }
            } else {
                None
            }
        } else {
            None
        };

        result.push(ReductionInfo {
            non_terminal: nt_name,
            action: rule.action.clone(),
            passthrough_index,
            rhs_symbols,
        });
    }

    Ok(result)
}

/// Get the type for a symbol (terminal payload type or non-terminal result type).
fn symbol_type(ctx: &CodegenContext, sym: &crate::lr::Symbol) -> Option<String> {
    ctx.grammar.types.get(&sym.id())?.clone()
}

fn determine_symbol_kind(ctx: &CodegenContext, sym: &crate::lr::Symbol) -> SymbolKind {
    let id = sym.id();
    if ctx.grammar.symbols.is_prec_terminal(id) {
        SymbolKind::PrecTerminal
    } else if ctx.grammar.symbols.is_terminal(id) {
        if ctx.grammar.types.get(&id).and_then(|t| t.as_ref()).is_some() {
            SymbolKind::PayloadTerminal
        } else {
            SymbolKind::UnitTerminal
        }
    } else {
        SymbolKind::NonTerminal
    }
}

/// Collect unique trait methods from all reductions.
pub fn collect_trait_methods(reductions: &[ReductionInfo]) -> Vec<TraitMethod> {
    let mut methods = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for info in reductions {
        if let AltAction::Named(method_name) = &info.action {
            // Use non_terminal + method_name as key to handle potential conflicts
            let key = format!("{}_{}", info.non_terminal, method_name);
            if seen.insert(key) {
                methods.push(TraitMethod {
                    name: method_name.clone(),
                    non_terminal: info.non_terminal.clone(),
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
    /// RHS symbols - typed ones become parameters.
    pub rhs_symbols: Vec<SymbolInfo>,
}

/// Extract indices of typed symbols from rhs_symbols.
pub fn typed_symbol_indices(rhs_symbols: &[SymbolInfo]) -> Vec<usize> {
    rhs_symbols.iter()
        .enumerate()
        .filter_map(|(i, s)| s.ty.as_ref().map(|_| i))
        .collect()
}
