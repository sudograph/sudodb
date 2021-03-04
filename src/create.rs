use crate::{
    ObjectTypeStore,
    FieldValueStore,
    FieldInput,
    SudodbError,
    FieldTypesStore
};
use std::collections::BTreeMap;

pub fn create(
    object_type_store: &mut ObjectTypeStore,
    object_type_name: &str,
    id: &str,
    inputs: Vec<FieldInput>
) -> Result<Vec<String>, SudodbError> {
    let object_type_result = object_type_store.get_mut(object_type_name);

    if let Some(object_type) = object_type_result {
        let mut field_values_map: FieldValueStore = BTreeMap::new();

        check_if_all_inputs_are_valid(
            object_type_name,
            &object_type.field_types_store,
            &inputs
        )?;

        for input in inputs {
            field_values_map.insert(input.field_name, input.field_value);
        }

        object_type.field_values_store.insert(String::from(id), field_values_map);

        return Ok(vec![]); // TODO this should return a string of the result
    }
    else {
        return Err(format!(
            "Object type {object_type_name} not found in database",
            object_type_name = object_type_name
        ));
    }
}

fn check_if_all_inputs_are_valid(
    object_type_name: &str,
    field_types_store: &FieldTypesStore,
    inputs: &Vec<FieldInput>
) -> Result<bool, SudodbError> {
    let invalid_inputs: Vec<&FieldInput> = inputs.iter().filter(|input| {
        return field_types_store.contains_key(&input.field_name) == false;
    }).collect();

    if invalid_inputs.len() == 0 {
        return Ok(true);
    }
    else {
        let invalid_input_field_names: Vec<String> = invalid_inputs.iter().map(|input| {
            return String::from(&input.field_name);
        }).collect();

        return Err(format!(
            "Tried to create fields that do not exist on object type {object_type_name}: {invalid_input_field_names}",
            object_type_name = object_type_name,
            invalid_input_field_names = invalid_input_field_names.join(",")
        ));
    }
}