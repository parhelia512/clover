use crate::runtime::opcode::Instruction;
use crate::runtime::object::Object;
use std::collections::HashMap;
use crate::runtime::assembly_information::{DebugInfo, FileInfo};
use crate::intermediate::Position;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub position: Position
}

impl RuntimeError {
    pub fn new(message: &str, position: Position) -> RuntimeError {
        RuntimeError {
            message: message.to_string(),
            position
        }
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    pub property_names: Vec<String>,
    pub functions: HashMap<String, usize>
}

impl Model {
    pub fn new() -> Model {
        Model {
            property_names: Vec::new(),
            functions: HashMap::new()
        }
    }

    pub fn add_property(&mut self, property_name: &str) {
        self.property_names.push(property_name.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameter_count: usize,
    pub local_count: usize,
    pub is_instance: bool,

    pub instructions: Vec<Instruction>
}

impl Function {
    pub fn new() -> Function {
        Function {
            parameter_count: 0,
            local_count: 0,
            is_instance: false,

            instructions: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub models: Vec<Model>,
    pub functions: Vec<Function>,
    pub constants: Vec<Object>,

    pub local_count: usize,

    // use to init local variable, key is local index, value is constant index
    pub local_values: HashMap<usize, usize>,

    // entry_point - 1 is the function index
    pub entry_point: usize,

    pub file_info: Option<FileInfo>,
    pub debug_info: Option<DebugInfo>
}