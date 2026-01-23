//! Terminal enum code generation.

use std::fmt::Write;

use super::table::TableData;
use super::CodegenContext;

/// Generate the terminal enum and its implementations.
pub fn generate(ctx: &CodegenContext, table_data: &TableData) -> String {
    let mut out = String::new();
    let vis = &ctx.visibility;
    let terminal_enum = format!("{}Terminal", ctx.name);
    let core = ctx.core_path();

    // Enum definition
    writeln!(out, "/// Terminal symbols for the parser.").unwrap();
    writeln!(out, "#[derive(Debug, Clone)]").unwrap();
    writeln!(out, "{} enum {} {{", vis, terminal_enum).unwrap();

    // Regular terminals
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            if let Some(ty) = payload_type {
                writeln!(out, "    {}({}),", variant_name, ty).unwrap();
            } else {
                writeln!(out, "    {},", variant_name).unwrap();
            }
        }
    }

    // Precedence terminals
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            if let Some(ty) = payload_type {
                writeln!(out, "    {}({}, {}::Precedence),", variant_name, ty, core).unwrap();
            } else {
                writeln!(out, "    {}({}::Precedence),", variant_name, core).unwrap();
            }
        }
    }

    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // impl block
    writeln!(out, "impl {} {{", terminal_enum).unwrap();

    // symbol_id method
    writeln!(out, "    /// Get the symbol ID for this terminal.").unwrap();
    writeln!(out, "    pub fn symbol_id(&self) -> {}::SymbolId {{", core).unwrap();
    writeln!(out, "        match self {{").unwrap();

    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            let table_id = table_data.terminal_ids.iter()
                .find(|(n, _)| n == name)
                .map(|(_, id)| *id)
                .unwrap_or(0);

            if payload_type.is_some() {
                writeln!(out, "            Self::{}(_) => {}::SymbolId({}),", variant_name, core, table_id).unwrap();
            } else {
                writeln!(out, "            Self::{} => {}::SymbolId({}),", variant_name, core, table_id).unwrap();
            }
        }
    }

    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            let table_id = table_data.terminal_ids.iter()
                .find(|(n, _)| n == name)
                .map(|(_, id)| *id)
                .unwrap_or(0);
            if payload_type.is_some() {
                writeln!(out, "            Self::{}(_, _) => {}::SymbolId({}),", variant_name, core, table_id).unwrap();
            } else {
                writeln!(out, "            Self::{}(_) => {}::SymbolId({}),", variant_name, core, table_id).unwrap();
            }
        }
    }

    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    // to_token method
    writeln!(out).unwrap();
    writeln!(out, "    /// Convert to a gazelle Token for parsing.").unwrap();
    writeln!(out, "    pub fn to_token(&self, symbol_ids: &impl Fn(&str) -> {}::SymbolId) -> {}::Token {{", core, core).unwrap();
    writeln!(out, "        match self {{").unwrap();

    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            if payload_type.is_some() {
                writeln!(out, "            Self::{}(_) => {}::Token::new(symbol_ids({:?}), {:?}),", variant_name, core, name, name).unwrap();
            } else {
                writeln!(out, "            Self::{} => {}::Token::new(symbol_ids({:?}), {:?}),", variant_name, core, name, name).unwrap();
            }
        }
    }

    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            if payload_type.is_some() {
                writeln!(out, "            Self::{}(_, prec) => {}::Token::with_prec(symbol_ids({:?}), {:?}, *prec),", variant_name, core, name, name).unwrap();
            } else {
                writeln!(out, "            Self::{}(prec) => {}::Token::with_prec(symbol_ids({:?}), {:?}, *prec),", variant_name, core, name, name).unwrap();
            }
        }
    }

    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    // precedence method for prec terminals
    writeln!(out).unwrap();
    writeln!(out, "    /// Get precedence for runtime precedence comparison.").unwrap();
    writeln!(out, "    /// Returns (level, assoc) where assoc: 0=left, 1=right.").unwrap();
    writeln!(out, "    pub fn precedence(&self) -> Option<(u8, u8)> {{").unwrap();
    writeln!(out, "        match self {{").unwrap();

    // Regular terminals have no precedence
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            if payload_type.is_some() {
                writeln!(out, "            Self::{}(_) => None,", variant_name).unwrap();
            } else {
                writeln!(out, "            Self::{} => None,", variant_name).unwrap();
            }
        }
    }

    // Prec terminals extract precedence from the Precedence type
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            if payload_type.is_some() {
                writeln!(out, "            Self::{}(_, prec) => Some((prec.level(), prec.assoc())),", variant_name).unwrap();
            } else {
                writeln!(out, "            Self::{}(prec) => Some((prec.level(), prec.assoc())),", variant_name).unwrap();
            }
        }
    }

    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(out, "}}").unwrap();

    out
}
