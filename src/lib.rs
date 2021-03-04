// TODO should we type field values?
// TODO how do we do transactions? Will the IC simply take care of that for us?
// TODO How much type checking and enforcing should sudodb do? Perhaps I should just leave that up to sudograph for now?

// TODO I think I should do some primitive type checking in here...such as if you try to update a field
// TODO that you did not initialize the type with...like creating or updating fields that you did not initialize the type with

use std::collections::BTreeMap;
mod read;

pub use read::read;

pub type ObjectTypeStore = BTreeMap<ObjectTypeName, ObjectType>;

type ObjectTypeName = String;
type FieldName = String;
type FieldValue = String; // TODO actually this needs to be generic
type PrimaryKey = String;

pub struct ObjectType {
    field_values_store: FieldValuesStore,
    // field_types: FieldTypeStore,
    // field_indexes: FieldIndexStore
    // TODO the indexes will go here
}

type FieldValuesStore = BTreeMap<PrimaryKey, FieldValueStore>;
type FieldValueStore = BTreeMap<FieldName, FieldValue>;

// enum FieldType {
//     String,
//     Number,
//     Boolean,
//     Date
// }

// type FieldTypeStore = BTreeMap<FieldName, FieldType>;
// type FieldIndexStore = BTreeMap<FieldValue, PrimaryKey>;

pub enum ReadInputType {
    Scalar,
    Relation
}

pub enum ReadInputOperation {
    Equals,
    Contains,
    In,
    StartsWith,
    EndsWith,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo
}

pub struct ReadInput {
    pub input_type: ReadInputType,
    pub input_operation: ReadInputOperation,
    pub field_name: String,
    pub field_value: String
    // TODO I think I will need the field type here
}

pub struct FieldInput {
    pub field_name: String,
    pub field_value: String // TODO this needs to be more generic somehow
}

pub fn init_object_type(
    object_type_store: &mut ObjectTypeStore,
    object_type_name: &str
) {
    println!("initObject");

    object_type_store.insert(String::from(object_type_name), ObjectType {
        field_values_store: BTreeMap::new()
    });
}

pub fn create(
    object_type_store: &mut ObjectTypeStore,
    object_type_name: &str,
    id: &str,
    inputs: Vec<FieldInput>
) -> Vec<String> {
    let object_type_result = object_type_store.get_mut(object_type_name);

    if let Some(object_type) = object_type_result {
        let mut field_values_map: FieldValueStore = BTreeMap::new();

        for input in inputs {
            field_values_map.insert(input.field_name, input.field_value);
        }

        object_type.field_values_store.insert(String::from(id), field_values_map);

        return vec![]; // TODO this should return a string of the result
    }
    else {
        return vec![];
    }
}

pub fn update(
    object_type_store: &mut ObjectTypeStore,
    object_type_name: &str,
    id: &str,
    inputs: Vec<FieldInput>
) -> Vec<String> {
    let object_type_result = object_type_store.get_mut(object_type_name);

    if let Some(object_type) = object_type_result {
        let field_values_map_result = object_type.field_values_store.get_mut(id);

        if let Some(field_values_map) = field_values_map_result {
            for input in inputs {
                field_values_map.insert(
                    input.field_name,
                    input.field_value
                );
            }
        
            return vec![]; // TODO this should return a string of the result
        }
        else {
            return vec![];
        }
    }
    else {
        return vec![];
    }
}

pub fn delete(
    object_type_store: &mut ObjectTypeStore,
    object_type_name: &str,
    id: &str
) -> Vec<String> {
    let object_type_result = object_type_store.get_mut(object_type_name);

    if let Some(object_type) = object_type_result {
        object_type.field_values_store.remove(id);

        return vec![]; // TODO this should return a string of the result
    }
    else {
        return vec![];
    }
}