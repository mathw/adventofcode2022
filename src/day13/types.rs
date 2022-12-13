#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    List(Vec<Value>),
    Integer(u32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Order {
    Correct,
    Incorrect,
}
