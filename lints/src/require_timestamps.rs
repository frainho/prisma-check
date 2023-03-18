use psl::{
    diagnostics::DatamodelError,
    schema_ast::ast::{SchemaAst, Top, WithSpan},
    Diagnostics,
};

use crate::Rule;

pub struct RequireTimestamps {}

impl Rule for RequireTimestamps {
    fn check(schema: &SchemaAst, diagnostics: &mut Diagnostics) {
        schema.iter_tops().for_each(|(_, top)| {
            if let Top::Model(model) = top {
                let timestamp_attributes_count = model
                    .iter_fields()
                    .filter(|(_, field)| field.name() == "createdAt" || field.name() == "updatedAt")
                    .collect::<Vec<_>>()
                    .len();

                if timestamp_attributes_count < 2 {
                    let error =
                        DatamodelError::new_invalid_model_error("Missing timestamps", model.span());
                    diagnostics.push_error(error)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use psl::Diagnostics;

    use crate::{require_timestamps::RequireTimestamps, Rule};

    #[test]
    fn it_passes_when_the_model_has_timestamps() {
        let mut diagnostics = Diagnostics::new();
        let schema = r#"
            model User {
              createdAt DateTime @default(now())
              updatedAt DateTime @updatedAt
            }
        "#;
        let schema_ast = psl::schema_ast::parse_schema(schema, &mut diagnostics);

        RequireTimestamps::check(&schema_ast, &mut diagnostics);

        assert_eq!(diagnostics.errors().len(), 0);
    }

    #[test]
    fn it_fails_when_the_model_does_not_have_timestamps() {
        let mut diagnostics = Diagnostics::new();
        let schema = r#"
            model User {
              id    Int     @id @default(autoincrement())
              email String  @unique
              name  String?
              posts Post[]
            }
        "#;
        let schema_ast = psl::schema_ast::parse_schema(schema, &mut diagnostics);

        RequireTimestamps::check(&schema_ast, &mut diagnostics);

        assert_eq!(diagnostics.errors().len(), 1);
    }
}
