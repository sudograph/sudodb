// TODO should we type field values?
// TODO how do we do transactions? Will the IC simply take care of that for us?
// TODO How much type checking and enforcing should sudodb do? Perhaps I should just leave that up to sudograph for now?

// TODO I think I should do some primitive type checking in here...such as if you try to update a field
// TODO that you did not initialize the type with...like creating or updating fields that you did not initialize the type with

use std::collections::BTreeMap;
mod read;
mod create;

pub use read::read;
pub use create::create;

pub type ObjectTypeStore = BTreeMap<ObjectTypeName, ObjectType>;

type ObjectTypeName = String;
type FieldName = String;
// type FieldValue = String; // TODO we need to get relations to work here, so we need to change things up a bit

type FieldValueScalar = String;

#[derive(Clone)]
pub enum FieldValue {
    Scalar(FieldValueScalar),
    Relation(FieldValueRelation)
}

enum FieldValueType {
    Scalar,
    Relation
}

#[derive(Clone)]
pub struct FieldValueRelation {
    pub relation_object_type_name: String,
    pub relation_primary_keys: Vec<PrimaryKey>
} // TODO do we need to know if this is a single or multiple relation?

type PrimaryKey = String;

pub struct ObjectType {
    field_values_store: FieldValuesStore,
    field_types_store: FieldTypesStore,
    // field_indexes: FieldIndexStore
    // TODO the indexes will go here
}

type FieldValuesStore = BTreeMap<PrimaryKey, FieldValueStore>;
type FieldValueStore = BTreeMap<FieldName, FieldValue>;
pub type FieldTypesStore = BTreeMap<FieldName, FieldType>;

// TODO time to get relations working!!!
pub enum FieldType {
    Boolean,
    Date,
    Float, // TODO do we need to split this into sizes? What should the default be?
    Int, // TODO do we need to split this into sizes? What should the default be?
    Relation(String), // TODO do we need single and multiple relations??
    String
}

// type FieldIndexStore = BTreeMap<FieldValue, PrimaryKey>;

pub enum ReadInputType {
    Scalar,
    Relation
}

pub enum ReadInputOperation {
    Contains,
    EndsWith,
    Equals,
    GreaterThan,
    GreaterThanOrEqualTo,
    In, // TODO this is just not implented for strings right now
    LessThan,
    LessThanOrEqualTo,
    StartsWith
    // TODO we have not implemented or yet, and we have not done arbitrarily nested ands and ors
}

pub struct ReadInput {
    pub input_type: ReadInputType, // TODO I think we might not need this
    pub input_operation: ReadInputOperation,
    pub field_name: String,
    pub field_value: String
    // TODO I think I will need the field type here
}

pub struct FieldInput {
    pub field_name: String,
    pub field_value: FieldValue
}

pub struct FieldTypeInput {
    pub field_name: String,
    pub field_type: FieldType
}

pub type SudodbError = String;

// TODO we should do some type checking on relations
// TODO it may be slightly difficult though, because we do not know the order the user will do relations in
// TODO perhaps, once done inserting into the map, just loop through and check that all relations are accounted for
// TODO keep a copy of the original or just abort/panic if there is a problem, this should roll back the state on the IC
pub fn init_object_type(
    object_type_store: &mut ObjectTypeStore,
    object_type_name: &str,
    field_type_inputs: Vec<FieldTypeInput>
) -> Result<(), SudodbError> {
    let mut field_types_store = BTreeMap::new();

    for field_type_input in field_type_inputs {
        field_types_store.insert(
            field_type_input.field_name,
            field_type_input.field_type
        );
    }

    object_type_store.insert(
        String::from(object_type_name),
        ObjectType {
            field_values_store: BTreeMap::new(),
            field_types_store
        }
    );

    return Ok(());
}

pub fn update(
    object_type_store: &mut ObjectTypeStore,
    object_type_name: &str,
    id: &str,
    inputs: Vec<FieldInput>
) -> Result<Vec<String>, SudodbError> {
    let object_type_result = object_type_store.get_mut(object_type_name);

    if let Some(object_type) = object_type_result {
        let field_values_map_result = object_type.field_values_store.get_mut(id);

        if let Some(field_values_map) = field_values_map_result {
            for input in inputs {
                // TODO simply respect relations here
                // field_values_map.insert(
                //     input.field_name,
                //     input.field_value
                // );
            }
        
            return Ok(vec![]); // TODO this should return a string of the result
        }
        else {
            return Err(format!(
                "record {id} not found for {object_type_name} object type",
                id = id,
                object_type_name = object_type_name
            ));
        }
    }
    else {
        return Err(format!(
            "{object_type_name} not found in database",
            object_type_name = object_type_name
        ));
    }
}

pub fn delete(
    object_type_store: &mut ObjectTypeStore,
    object_type_name: &str,
    id: &str
) -> Result<Vec<String>, SudodbError> {
    let object_type_result = object_type_store.get_mut(object_type_name);

    if let Some(object_type) = object_type_result {
        object_type.field_values_store.remove(id);

        return Ok(vec![]); // TODO this should return a string of the result
    }
    else {
        return Err(format!(
            "{object_type_name} not found in database",
            object_type_name = object_type_name
        ));
    }
}