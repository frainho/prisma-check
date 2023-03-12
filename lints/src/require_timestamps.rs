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
