use psl::{schema_ast::ast::SchemaAst, Diagnostics};

pub mod require_snake_case_map;
pub mod require_timestamps;

pub trait Rule {
    fn check(schema: &SchemaAst, diagnostics: &mut Diagnostics);
}
