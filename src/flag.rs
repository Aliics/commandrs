use std::any::TypeId;

#[derive(PartialEq, Debug)]
pub struct Flag {
    pub name: String,
    pub is_required: bool,
    pub type_id: TypeId,
}

#[derive(PartialEq, Debug)]
pub struct FlagValue {
    pub name: String,
    pub str_value: String,
}
