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
