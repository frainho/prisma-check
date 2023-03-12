use std::{fs::File, io::Read};

use clap::{command, Parser};
use lints::{
    require_snake_case_map::RequireSnakeCaseMap, require_timestamps::RequireTimestamps, Rule,
};
use psl::Diagnostics;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to schema.prisma
    #[arg(short, long, default_value_t = String::from("prisma/schema.prisma"))]
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;

    let mut schema_file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to open prisma schema file due to: {}", e);
        }
    };

    let mut schema = String::new();
    if let Err(error) = schema_file.read_to_string(&mut schema) {
        panic!("Failed to read prisma schema file due to: {}", error);
    };

    let mut diagnostics = Diagnostics::new();
    let schema_ast = psl::schema_ast::parse_schema(&schema, &mut diagnostics);

    RequireTimestamps::check(&schema_ast, &mut diagnostics);
    RequireSnakeCaseMap::check(&schema_ast, &mut diagnostics);

    let mut message: Vec<u8> = Vec::new();
    for error in diagnostics.errors() {
        error
            .pretty_print(&mut message, "schema.prisma", &schema)
            .unwrap();
    }

    println!("{}", String::from_utf8(message).unwrap());
}
