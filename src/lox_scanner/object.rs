
#[derive (Debug, PartialEq)]
pub enum Object {
    STRING(String),
    NUMBER(f64),
    NULL,
}
