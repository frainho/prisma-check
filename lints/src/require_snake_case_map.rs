use cruet::is_snake_case;
use psl::{
    diagnostics::DatamodelError,
    schema_ast::ast::{SchemaAst, Top, WithName, WithSpan},
    Diagnostics,
};

use crate::Rule;

pub struct RequireSnakeCaseMap {}

impl Rule for RequireSnakeCaseMap {
    fn check(schema: &SchemaAst, diagnostics: &mut Diagnostics) {
        schema.iter_tops().for_each(|(_, top)| {
            if let Top::Model(model) = top {
                model.iter_fields().for_each(|(_, field)| {
                    if is_snake_case(field.name()) {
                        return;
                    }

                    let map_attribute = field
                        .attributes
                        .iter()
                        .find(|attribute| attribute.name() == "map");

                    if let Some(map_attribute) = map_attribute {
                        if let Some(argument) = map_attribute.arguments.arguments.first() {
                            if let Some((value, _)) = argument.value.as_string_value() {
                                if !is_snake_case(value) {
                                    let error = DatamodelError::new_attribute_validation_error(
                                        "Map attribute is not camel case",
                                        "Map",
                                        field.span(),
                                    );
                                    diagnostics.push_error(error);
                                }
                            } else {
                                let error = DatamodelError::new_attribute_validation_error(
                                    "Attribute has incorrect value, expected string",
                                    "Map",
                                    field.span(),
                                );
                                diagnostics.push_error(error);
                            }
                        } else {
                            let error = DatamodelError::new_attribute_validation_error(
                                "Attribute is empty",
                                "Map",
                                field.span(),
                            );
                            diagnostics.push_error(error);
                        }
                    } else {
                        let error = DatamodelError::new_attribute_validation_error(
                            "Missing attribute",
                            "Map",
                            field.span(),
                        );
                        diagnostics.push_error(error);
                    }
                });
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use psl::Diagnostics;

    use crate::{require_snake_case_map::RequireSnakeCaseMap, Rule};

    #[test]
    fn it_does_not_detect_errors_when_all_fields_are_snake_case() {
        let mut diagnostics = Diagnostics::new();
        let schema = r#"
            model User {
              id    Int     @id @default(autoincrement())
              email String  @unique
              name  String?
              posts Post[]
              createdAt DateTime @default(now()) @map("created_at")
              updatedAt DateTime @updatedAt @map("created_at")
            }
        "#;
        let schema_ast = psl::schema_ast::parse_schema(schema, &mut diagnostics);

        RequireSnakeCaseMap::check(&schema_ast, &mut diagnostics);

        assert_eq!(diagnostics.errors().len(), 0);
    }

    #[test]
    fn it_does_not_complain_with_a_field_with_a_single_word() {
        let mut diagnostics = Diagnostics::new();
        let schema = r#"
            model User {
              email String @unique
            }
        "#;
        let schema_ast = psl::schema_ast::parse_schema(schema, &mut diagnostics);

        RequireSnakeCaseMap::check(&schema_ast, &mut diagnostics);

        assert_eq!(diagnostics.errors().len(), 0);
    }

    #[test]
    fn it_complains_when_a_field_is_not_snake_case() {
        let mut diagnostics = Diagnostics::new();
        let schema = r#"
            model User {
              createdAt String @unique
            }
        "#;
        let schema_ast = psl::schema_ast::parse_schema(schema, &mut diagnostics);

        RequireSnakeCaseMap::check(&schema_ast, &mut diagnostics);

        assert_eq!(diagnostics.errors().len(), 1);
    }

    #[test]
    fn it_complains_when_a_field_has_an_invalid_map_attribute() {
        let mut diagnostics = Diagnostics::new();
        let schema = r#"
            model User {
              createdAt String @unique @map()
            }
        "#;
        let schema_ast = psl::schema_ast::parse_schema(schema, &mut diagnostics);

        RequireSnakeCaseMap::check(&schema_ast, &mut diagnostics);

        assert_eq!(diagnostics.errors().len(), 1);
    }

    #[test]
    fn it_complains_when_the_map_attribute_is_not_snake_case() {
        let mut diagnostics = Diagnostics::new();
        let schema = r#"
            model User {
              createdAt String @unique @map("createdAt")
            }
        "#;
        let schema_ast = psl::schema_ast::parse_schema(schema, &mut diagnostics);

        RequireSnakeCaseMap::check(&schema_ast, &mut diagnostics);

        assert_eq!(diagnostics.errors().len(), 1);
    }
}
