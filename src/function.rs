use crate::chunk::Chunk;

#[derive(Debug, Clone)]
pub enum FunctionType {
    Fn,
    Script,
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub num_params: usize,
    pub chunk: Chunk<'a>,
    pub name: String,
    pub native: bool,
    pub function_type: FunctionType,
}

impl <'a> Function <'a> {
    pub fn new(name: String, function_type: FunctionType) -> Function<'a> {
        Function {
            num_params: 0,
            chunk: Chunk::new(),
            name,
            native: false,
            function_type,
        }
    }
}
