use std::any::TypeId;

#[derive(PartialEq, Debug)]
pub struct Flag<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub is_required: bool,
    pub type_id: TypeId,
}

#[derive(PartialEq, Debug)]
pub struct FlagValue<'a> {
    pub name: &'a str,
    pub str_value: String,
}
