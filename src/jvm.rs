//! Lightweight implementation of a parser and decoder for JVM class files.
use byteorder::{BigEndian, ReadBytesExt};
use std::collections::HashMap;



use std::io::{Cursor, Read};
use std::path::{Path};
use std::{io};

/// Values of magic bytes of a JVM class file.
const JVM_CLASS_FILE_MAGIC: u32 = 0xCAFEBABE;

/// `CPInfo` represents constant pool entries.
#[derive(Debug, Clone)]
struct CPInfo {
    // Value of `ConstantKind` indicates the kind of the constant represented
    // by this entry.
    tag: u8,
    info: Vec<u8>,
}

/// `ConstantKind` encodes the kind of a constant in the constants pool.
#[derive(Debug, Copy, Clone)]
enum ConstantKind {
    Class = 7,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodref = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

/// Verification type specifies the type of a single variable location or
/// a single operand stack entry.
#[derive(Debug, Copy, Clone)]
enum VerificationType {
    TopVerification = 0,
    IntegerVerification = 1,
    FloatVerification = 2,
    LongVerification = 4,
    DoubleVerification = 3,
    NullVerification = 5,
    UninitializedThisVerification = 6,
    ObjectVerification = 7,
    UninitializedVerification = 8,
}

/// Verification info struct.
#[derive(Debug, Copy, Clone)]
struct VerificationInfo {
    tag: VerificationType,
    cpool_index_or_offset: u16,
}

/// Stack map frame type.
#[derive(Debug, Copy, Clone)]
enum StackMapFrameType {
    Same,
    SameLocals,
    SameLocalsExtended,
    Chop,
    SameExtended,
    Append,
    Full,
}

/// Stack map frame.
#[derive(Debug, Clone)]
struct StackMapFrame {
    t: StackMapFrameType,
    offset_delta: u16,
    locals: Vec<VerificationInfo>,
    stack: Vec<VerificationInfo>,
}

/// Bootstrap method.
#[derive(Debug, Clone)]
struct BootstrapMethod {
    method_ref: u16,
    arguments: Vec<u16>,
}

#[derive(Debug, Clone)]
enum AttributeInfo {
    ConstantValueAttribute {
        constant_value_index: u64,
        attribute_name: String,
    },
    CodeAttribute {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        // Exception table.
        attributes: HashMap<String, AttributeInfo>,
        attribute_name: String,
    },
    StackMapTableAttribute {
        entries: Vec<StackMapFrame>,
        attribute_name: String,
    },
    SourceFileAttribute {
        source_file_index: u16,
        attribute_name: String,
    },
    BootstrapMethodsAttribute {
        bootstrap_methods: Vec<BootstrapMethod>,
        attribute_name: String,
    },
    NestHostAttribute {
        host_class_index: u16,
        attribute_name: String,
    },
    NestMembersAttribute {
        classes: Vec<u16>,
        attribute_name: String,
    },
}

#[derive(Debug, Clone)]
struct FieldInfo {
    access_flag: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: HashMap<String, AttributeInfo>,
}

#[derive(Debug, Clone)]
struct MethodInfo {
    access_flag: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: HashMap<String, AttributeInfo>,
}

/// `JVMClassFile` represents a Java class file.
#[derive(Debug, Clone)]
pub struct JVMClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<CPInfo>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    fields: Vec<FieldInfo>,
    methods_count: u16,
    methods: Vec<MethodInfo>,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

/// `JVMParser` namespaces functions that handle parsing of Java class files.
#[derive(Debug)]
pub struct JVMParser;

impl JVMParser {
    #[must_use]
    // Creates a new JVMParser with a given Java class file to parse.
    pub fn new() -> Self {
        Self {}
    }

    // Parse a preloaded Java class file.
    fn parse(&self, class_file_bytes: &[u8]) -> io::Result<JVMClassFile> {
        // Create a new cursor on the class file bytes.
        let mut buffer = Cursor::new(class_file_bytes);
        // Start reading data.
        let magic = buffer.read_u32::<BigEndian>()?;
        let minor_version = buffer.read_u16::<BigEndian>()?;
        let major_version = buffer.read_u16::<BigEndian>()?;

        let jvm_class_file = JVMClassFile {
            magic: magic,
            minor_version: minor_version,
            major_version: major_version,
            constant_pool_count: 0,
            constant_pool: Vec::new(),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces_count: 0,
            interfaces: Vec::new(),
            fields_count: 0,
            fields: Vec::new(),
            methods_count: 0,
            methods: Vec::new(),
            attributes_count: 0,
            attributes: Vec::new(),
        };
        Ok(jvm_class_file)
    }
}

/// Helper function to read file into a buffer.
fn read_class_file(fp: &Path) -> Vec<u8> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(fp).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::Path;

    #[test]
    fn can_you_read_class_file() {
        let env_var = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(&env_var).join("support/SingleFuncCall.class");
        let class_file_bytes = read_class_file(&path);
        let class_file = JVMParser::new().parse(&class_file_bytes);

        assert!(class_file.is_ok());
        assert_eq!(JVM_CLASS_FILE_MAGIC, class_file.unwrap().magic,);
    }

    #[test]
    fn can_parse_class_file_header() {}
}
