use lints::{
    require_snake_case_map::RequireSnakeCaseMap, require_timestamps::RequireTimestamps, Rule,
};
use psl::Diagnostics;

fn main() {
    let schema = include_str!("schema.prisma");
    let mut diagnostics = Diagnostics::new();
    let schema_ast = psl::schema_ast::parse_schema(schema, &mut diagnostics);

    RequireTimestamps::check(&schema_ast, &mut diagnostics);
    RequireSnakeCaseMap::check(&schema_ast, &mut diagnostics);

    let mut message: Vec<u8> = Vec::new();
    for error in diagnostics.errors() {
        error
            .pretty_print(&mut message, "schema.prisma", schema)
            .unwrap();
    }

    println!("{}", String::from_utf8(message).unwrap());
}
