//! Reduction analysis for code generation.

use crate::lr::AltAction;
use super::CodegenContext;

/// Information about a reduction for code generation.
#[derive(Debug, Clone)]
pub struct ReductionInfo {
    /// The non-terminal name (LHS).
    pub non_terminal: String,
    /// The action for this reduction (from the grammar rule).
    pub action: AltAction,
    /// Variant name for enum generation (from @name).
    pub variant_name: Option<String>,
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
        let is_synthetic = nt_name.starts_with("__");

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

        // Determine variant name
        let variant_name = match &rule.action {
            AltAction::Named(name) if !is_synthetic && !name.is_empty() => Some(name.clone()),
            _ => None,
        };

        result.push(ReductionInfo {
            non_terminal: nt_name,
            action: rule.action.clone(),
            variant_name,
            rhs_symbols,
        });
    }

    // Deduplicate variant names within each non-terminal
    let mut nt_counts: std::collections::HashMap<String, std::collections::HashMap<String, usize>> = std::collections::HashMap::new();
    for info in &mut result {
        if let Some(ref name) = info.variant_name {
            let counts = nt_counts.entry(info.non_terminal.clone()).or_default();
            let count = counts.entry(name.clone()).or_insert(0);
            if *count > 0 {
                info.variant_name = Some(format!("{}{}", name, count));
            }
            *count += 1;
        }
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

/// Extract indices of typed symbols from rhs_symbols.
pub fn typed_symbol_indices(rhs_symbols: &[SymbolInfo]) -> Vec<usize> {
    rhs_symbols.iter()
        .enumerate()
        .filter_map(|(i, s)| s.ty.as_ref().map(|_| i))
        .collect()
}
