use crate::{
    ObjectTypeStore,
    ReadInput,
    FieldValuesStore,
    FieldValueStore,
    ReadInputOperation
};

// TODO figure out string interpolation better to clean this function up
// TODO this is where all of the complexity will lie, or much of at at least
// TODO we need to figure out how to enable amazing filtering capabilities
pub fn read(
    object_type_store: &ObjectTypeStore,
    object_type_name: &str,
    inputs: Vec<ReadInput>
) -> Vec<String> { // TODO I think I want this to return a list of JSON strings...GraphQL can handle type checking the actual values I hope
    let object_type_result = object_type_store.get(object_type_name);

    if let Some(object_type) = object_type_result {
        let field_value_stores = find_field_value_stores_for_inputs(
            &object_type.field_values_store,
            &inputs
        );

        let field_value_store_strings = field_value_stores.iter().map(|field_value_store| {
            return convert_field_value_store_to_json_string(field_value_store);
        }).collect();
    
        return field_value_store_strings;
    }
    else {
        return vec![];
    }
}

// TODO I believe the result in the fold here needs to be mutable for efficiency...not sure, but perhaps
fn find_field_value_stores_for_inputs(
    field_values_store: &FieldValuesStore,
    inputs: &Vec<ReadInput>
) -> Vec<FieldValueStore> {
    let field_value_stores = field_values_store.values().fold(vec![], |mut result, field_value_store| {
        let inputs_match = field_value_store_matches_inputs(
            &field_value_store,
            &inputs
        );

        if inputs_match == true {
            result.push(field_value_store.clone());

            return result;
        }
        else {
            return result;
        }
    });

    return field_value_stores;
}

// TODO we still need to implement relations and operations based on type...
// TODO we will probably want to match first on the type of the input, then have a function to implement
// TODO all operations for each type
fn field_value_store_matches_inputs(
    field_value_store: &FieldValueStore,
    inputs: &Vec<ReadInput>
) -> bool {
    return inputs.iter().all(|input| {
        if let Some(field_value) = field_value_store.get(&input.field_name) {
            match input.input_operation {
                ReadInputOperation::Contains => {
                    return field_value.contains(&input.field_value);
                },
                ReadInputOperation::EndsWith => {
                    return field_value.ends_with(&input.field_value);
                },
                ReadInputOperation::Equals => {
                    return field_value == &input.field_value;
                },
                ReadInputOperation::GreaterThan => {
                    return field_value > &input.field_value;
                },
                ReadInputOperation::GreaterThanOrEqualTo => {
                    return field_value >= &input.field_value;
                },
                ReadInputOperation::In => {
                    return false; // TODO this is just not implented for strings right now
                },
                ReadInputOperation::LessThan => {
                    return field_value < &input.field_value;
                },
                ReadInputOperation::LessThanOrEqualTo => {
                    return field_value <= &input.field_value;
                },
                ReadInputOperation::StartsWith => {
                    return field_value.starts_with(&input.field_value);
                }
            };
        }
        else {
            return false;
        }
    });
}

// TODO this only works for string values right now, and only scalar values as well
// TODO We will need to add support for numbers, null, undefined, and relations
fn convert_field_value_store_to_json_string(field_value_store: &FieldValueStore) -> String {
    let inner_json = field_value_store.iter().enumerate().fold(String::from(""), |result, (i, (key, value))| {
        return format!(
            "{result}\"{key}\":\"{value}\"{comma}",
            result = result,
            key = key,
            value = value,
            comma = if i == field_value_store.iter().len() - 1 { "" } else { "," }
        );
    });

    let full_json = format!(
        "{{{inner_json}}}",
        inner_json = inner_json
    );

    return full_json;
}