//! Abstract representation of a Java program.
use crate::jvm::{AttributeInfo, CPInfo, JVMClassFile, StackMapFrame};

use regex::Regex;

/// Primitive types supported by the JVM.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BaseTypeKind {
    Int,
    Long,
    Float,
    Double,
    Void,
    String,
    List,
}

/// JVM value type.
#[derive(Debug, Clone)]
pub struct Type {
    t: BaseTypeKind,
    sub_t: Option<Box<Type>>,
}

impl Type {
    /// Empty constructor, we could use `Default` but hey.
    pub fn new() -> Self {
        Self {
            t: BaseTypeKind::Int,
            sub_t: None,
        }
    }
    /// Returns the size in WORD (4 bytes) of a given type.
    pub fn size(&self) -> usize {
        match self.t {
            BaseTypeKind::Int | BaseTypeKind::Float => 1,
            BaseTypeKind::Long | BaseTypeKind::Double => 2,
            _ => 0,
        }
    }
}

/// Representation of Java programs that we want to run.
#[derive(Debug, Clone)]
pub struct Program {
    // Constant pool.
    pub constant_pool: Vec<CPInfo>,
    // Methods.
    // pub methods: HashMap<usize, Method>,
    pub methods: Vec<Method>,
}

/// Java class method representation for the interpreter.
#[derive(Debug, Clone)]
pub struct Method {
    _name_index: u16,
    _return_type: Type,
    pub arg_types: Vec<Type>,
    _max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    _constant: Option<u16>,
    _stack_map_table: Option<Vec<StackMapFrame>>,
}

impl Default for Method {
    fn default() -> Self {
        Self {
            _name_index: 0,
            _return_type: Type::new(),
            arg_types: Vec::new(),
            _max_stack: 0,
            max_locals: 0,
            code: Vec::new(),
            _constant: None,
            _stack_map_table: None,
        }
    }
}

impl Program {
    /// Build a new program from a parsed class file.
    /// # Panics
    /// Can panic if class file is missing Code attribute.
    #[must_use]
    pub fn new(class_file: &JVMClassFile) -> Self {
        let constants = class_file.constant_pool();
        // let mut methods: HashMap<usize, Method> = HashMap::new();
        let mut methods: Vec<Method> = vec![Method::default(); 256];
        for method_info in &class_file.methods() {
            let mut arg_types: Vec<Type> = Vec::new();
            let mut return_type: Type = Type {
                t: BaseTypeKind::Void,
                sub_t: None,
            };
            let descriptor =
                &constants[method_info.descriptor_index() as usize];
            let _method_name = &constants[method_info.name_index() as usize];

            if let CPInfo::ConstantUtf8 { bytes } = descriptor {
                (arg_types, return_type) = Self::parse_method_types(bytes);
            }
            let attr = method_info.attributes();

            let (max_stack, max_locals, code) =
                if let Some(AttributeInfo::CodeAttribute {
                    max_stack,
                    max_locals,
                    code,
                    ..
                }) = attr.get("Code")
                {
                    (*max_stack, *max_locals, code.clone())
                } else {
                    panic!("Expected at least one code attribute")
                };

            let constant =
                if let Some(AttributeInfo::ConstantValueAttribute {
                    constant_value_index,
                    ..
                }) = attr.get("ConstantValue")
                {
                    Some(*constant_value_index)
                } else {
                    None
                };

            let stack_map_table =
                if let Some(AttributeInfo::StackMapTableAttribute {
                    entries,
                    ..
                }) = attr.get("StackMapTable")
                {
                    Some(entries.clone())
                } else {
                    None
                };

            let method = Method {
                _name_index: method_info.name_index(),
                _return_type: return_type,
                arg_types,
                _max_stack: max_stack,
                max_locals,
                code,
                _constant: constant,
                _stack_map_table: stack_map_table,
            };
            // methods.insert(method_info.name_index() as usize, method);
            methods[method_info.name_index() as usize] = method;
        }

        Self {
            // Get a copy of the constant pool.
            constant_pool: class_file.constant_pool(),
            // Get a copy of the program methods.
            methods,
        }
    }

    // Find method name index in the constant pool by reference.
    pub fn find_method(&self, method_ref: usize) -> i32 {
        match self.constant_pool[method_ref] {
            CPInfo::ConstantMethodRef {
                name_and_type_index,
                ..
            } => {
                if let CPInfo::ConstantNameAndType { name_index, .. } =
                    self.constant_pool[name_and_type_index as usize]
                {
                    return name_index.into();
                }
                0
            }
            _ => panic!("Expected ConstantMethodRef"),
        }
    }

    // Returns program entry point, in this case the index of the method
    // main.
    pub fn entry_point(&self) -> usize {
        for (index, _) in self.methods.iter().enumerate() {
            match self.constant_pool.get(index) {
                Some(constant) => {
                    if let CPInfo::ConstantUtf8 { bytes } = constant {
                        if bytes == "main" {
                            return index;
                        }
                    }
                }
                None => panic!("method \"main\" was not found"),
            }
        }
        // This might cause some issues but since the input to our runtime
        // is a class file that already passed the Java compiler we should
        // assume a main function already exists.
        0
    }

    // Returns a slice containing code of method pointed at by `method_index`.
    pub fn code(&self, method_index: usize) -> &[u8] {
        &self.methods[method_index].code
    }

    // Return the declared max locals for a method.
    pub fn max_locals(&self, method_index: usize) -> u16 {
        self.methods[method_index].max_locals
    }

    // Parse constant method types, returns a tuple of argument types and
    // return types.
    fn parse_method_types(bytes: &str) -> (Vec<Type>, Type) {
        let re = Regex::new(r"\(([^\)]*)\)([^$]+)").unwrap();
        let caps = re.captures(bytes).unwrap();
        let arg_string = caps.get(1).map_or("", |m| return m.as_str());
        let return_type_string = caps.get(2).map_or("", |m| return m.as_str());
        let mut types: Vec<Type> = Vec::new();
        let ret_type = Self::decode_type(return_type_string);

        let mut arg_string_slice = arg_string;
        while !arg_string_slice.is_empty() {
            let t = Self::decode_type(arg_string_slice);
            types.push(t.clone());
            let length = Self::decode_type_string_length(&t);
            arg_string_slice = substr(
                arg_string_slice,
                length,
                arg_string_slice.len() - length,
            );
        }
        (types, ret_type)
    }

    /// Returns the type's string representation length.
    /// # Panics
    /// Function panics if class file has invalid representation for a list
    /// type.
    #[must_use]
    pub fn decode_type_string_length(t: &Type) -> usize {
        match t.t {
            BaseTypeKind::String => 18,
            BaseTypeKind::List => {
                return 1 + Self::decode_type_string_length(
                    t.sub_t.as_ref().unwrap(),
                )
            }
            _ => 1,
        }
    }

    /// Returns the Java equivalent type from a type's string representation.
    #[must_use]
    pub fn decode_type(type_str: &str) -> Type {
        match &type_str[0..1] {
            "I" => Type {
                t: BaseTypeKind::Int,
                sub_t: None,
            },
            "J" => Type {
                t: BaseTypeKind::Long,
                sub_t: None,
            },
            "F" => Type {
                t: BaseTypeKind::Float,
                sub_t: None,
            },
            "D" => Type {
                t: BaseTypeKind::Double,
                sub_t: None,
            },
            "V" => Type {
                t: BaseTypeKind::Void,
                sub_t: None,
            },
            "[" => {
                let st = Self::decode_type(&type_str[1..(type_str.len() - 1)]);
                let subtype = Type {
                    t: st.t,
                    sub_t: st.sub_t,
                };
                Type {
                    t: BaseTypeKind::List,
                    sub_t: Some(Box::new(subtype)),
                }
            }
            // We can support byte, char... later
            _ => Type {
                t: BaseTypeKind::String,
                sub_t: None,
            },
        }
    }
}

fn substr(s: &str, start: usize, length: usize) -> &str {
    let end = start + length;
    &s[start..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::jvm::{read_class_file, JVMParser};
    use std::env;
    use std::path::Path;

    #[test]
    fn can_build_program() {
        let env_var = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(&env_var).join("support/tests/Factorial.class");
        let class_file_bytes = read_class_file(&path).unwrap_or_else(|_| {
            panic!("Failed to parse file : {:?}", path.as_os_str())
        });
        let result = JVMParser::parse(&class_file_bytes);
        assert!(result.is_ok());
        let class_file = result.unwrap();
        let program = Program::new(&class_file);

        let methods = vec![
            Method {
                _name_index: 27,
                _return_type: Type {
                    t: BaseTypeKind::Void,
                    sub_t: None,
                },
                arg_types: vec![Type {
                    t: BaseTypeKind::List,
                    sub_t: Some(Box::new(Type {
                        t: BaseTypeKind::String,
                        sub_t: None,
                    })),
                }],
                _max_stack: 2,
                max_locals: 2,
                code: vec![
                    16, 12, 184, 0, 7, 60, 178, 0, 13, 27, 182, 0, 19, 177,
                ],
                _constant: None,
                _stack_map_table: None,
            },
            Method {
                _name_index: 5,
                _return_type: Type {
                    t: BaseTypeKind::Void,
                    sub_t: None,
                },
                arg_types: vec![],
                _max_stack: 1,
                max_locals: 1,
                code: vec![42, 183, 0, 1, 177],
                _constant: None,
                _stack_map_table: None,
            },
            Method {
                _name_index: 11,
                _return_type: Type {
                    t: BaseTypeKind::Int,
                    sub_t: None,
                },
                arg_types: vec![Type {
                    t: BaseTypeKind::Int,
                    sub_t: None,
                }],
                _max_stack: 2,
                max_locals: 3,
                code: vec![
                    4, 60, 5, 61, 28, 26, 163, 0, 13, 27, 28, 104, 60, 132, 2,
                    1, 167, 255, 244, 27, 172,
                ],
                _constant: None,
                _stack_map_table: None,
            },
        ];

        for method in methods {
            let name_index = method._name_index;
            let program_method = &program.methods[name_index as usize];
            assert_eq!(method.code, program_method.code);
        }
        assert_eq!(program.entry_point(), 27);
    }
}
