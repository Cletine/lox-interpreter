
#[derive (Debug, PartialEq, Clone)]
pub enum Object {
    STRING(String),
    NUMBER(f64),
    NULL,
}
