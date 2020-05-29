use std::collections::BTreeSet;

use super::{contains_eventstreams, mutate_type_name};
use crate::botocore::{Shape, ShapeType};
use crate::Service;

pub fn filter_types(service: &Service<'_>) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut deserialized_types: BTreeSet<String> = BTreeSet::new();
    let mut serialized_types: BTreeSet<String> = BTreeSet::new();
    for operation in service.operations().values() {
        if let Some(ref output) = operation.output {
            let output_shape = service
                .get_shape(&output.shape)
                .expect("Shape type missing from service definition");

            if !can_skip_deserializer_recursively(service, output_shape) {
                recurse_find_serializable_shapes(service, &mut deserialized_types, &output.shape);
            }
        }
        if let Some(ref input) = operation.input {
            recurse_find_serializable_shapes(service, &mut serialized_types, &input.shape);
        }
    }

    (serialized_types, deserialized_types)
}

fn recurse_find_serializable_shapes(
    service: &Service<'_>,
    types: &mut BTreeSet<String>,
    shape_name: &str,
) {
    let shape = service
        .get_shape(shape_name)
        .expect("Shape type missing from service definition");

    if !contains_eventstreams(service, &shape) {
        // Event stream container is not plain data because it references an active request
        // and therefore cannot be serialized.
        types.insert(mutate_type_name(service, shape_name).to_owned());
    }

    match shape.shape_type {
        ShapeType::Structure => {
            if let Some(ref members) = shape.members {
                for member in members.values() {
                    if Some(true) != member.deprecated
                        && member.location != Some("header".to_owned())
                        && member.location != Some("headers".to_owned())
                        && !types.contains(&member.shape)
                    {
                        recurse_find_serializable_shapes(service, types, &member.shape);
                    }
                }
            }
        }
        ShapeType::Map => {
            recurse_find_serializable_shapes(service, types, shape.key_type());
            recurse_find_serializable_shapes(service, types, shape.value_type());
        }
        ShapeType::List => {
            recurse_find_serializable_shapes(service, types, shape.member_type());
        }
        _ => {}
    }
}

fn can_skip_deserializer_recursively(service: &Service<'_>, output_shape: &Shape) -> bool {
    if let Some(ref payload_field) = output_shape.payload {
        let payload_member = output_shape
            .members
            .as_ref()
            .unwrap()
            .get(payload_field)
            .unwrap();
        let payload_shape = service.get_shape(&payload_member.shape).unwrap();

        // we can skip deserializer generation if the payload member in the output shape
        // is explicitly flagged as streaming, or if the payload shape itself is
        // a Blob or a String (in which case it will just be the raw response body)
        payload_member.streaming()
            || payload_shape.shape_type == ShapeType::Blob
            || payload_shape.shape_type == ShapeType::String
    } else {
        false
    }
}
