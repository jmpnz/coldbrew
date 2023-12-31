//! JVM bytecode definitions.
use std::fmt;

/// OPCodes supported by the JVM as documented in the spec document.
/// ref: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-7.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OPCode {
    /// Nop designates a no operation, it's similar to a NOP (0x90).
    Nop,
    /// Push `null` into the stack.
    AConstNull,
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Lconst0,
    Lconst1,
    Fconst0,
    Fconst1,
    Fconst2,
    Dconst0,
    Dconst1,
    /// Push a single byte operand into the stack.
    BiPush,
    /// Push a two byte operand (short) into the stack.
    SiPush,
    /// Push an `int` or `float` value from the runtime constant pool at the
    /// given index (byte long) into the stack.
    Ldc,
    /// Push an `int` or `float` value from the runtime constant pool at the
    /// given index (two byte long) into the stack.
    LdcW,
    /// Push a `long` or `double` value from the runtime constant pool at the
    /// given index into the stack.
    Ldc2W,
    /// Load an `int` from the local variables array of the current frame
    /// and push it into the stack, the index is given as an operand.
    ILoad,
    /// Load a `long` from the local variables array of the current frame
    /// and push it into the stack, the index is given as an operand.
    LLoad,
    /// Load a `float` from the local variables array of the current frame
    /// and push it into the stack, the index is given as an operand.
    FLoad,
    /// Load a `double` from the local variables array of the current frame
    /// and push it into the stack, the index is given as an operand.
    DLoad,
    /// Load a `reference` from a local variable.
    ALoad,
    /// Load `int` at index 0 from the local variables array of the current
    /// frame and push it into the stack.
    ILoad0,
    /// Load `int` at index 1 from the local variables array of the current
    /// frame and push it into the stack.
    ILoad1,
    /// Load `int` at index 2 from the local variables array of the current
    /// frame and push it into the stack.
    ILoad2,
    /// Load `int` at index 3 from the local variables array of the current
    /// frame and push it into the stack.
    ILoad3,
    /// Load `long` at index 0 from the local variables array of the current
    /// frame and push it into the stack.
    LLoad0,
    /// Load `long` at index 1 from the local variables array of the current
    /// frame and push it into the stack.
    LLoad1,
    /// Load `long` at index 2 from the local variables array of the current
    /// frame and push it into the stack.
    LLoad2,
    /// Load `long` at index 3 from the local variables array of the current
    /// frame and push it into the stack.
    LLoad3,
    /// Load `float` at index 0 from the local variables array of the current
    /// frame and push it into the stack.
    FLoad0,
    /// Load `float` at index 1 from the local variables array of the current
    /// frame and push it into the stack.
    FLoad1,
    /// Load `float` at index 2 from the local variables array of the current
    /// frame and push it into the stack.
    FLoad2,
    /// Load `float` at index 3 from the local variables array of the current
    /// frame and push it into the stack.
    FLoad3,
    /// Load `double` at index 0 from the local variables array of the current
    /// frame and push it into the stack.
    DLoad0,
    /// Load `double` at index 1 from the local variables array of the current
    /// frame and push it into the stack.
    DLoad1,
    /// Load `double` at index 2 from the local variables array of the current
    /// frame and push it into the stack.
    DLoad2,
    /// Load `double` at index 3 from the local variables array of the current
    /// frame and push it into the stack.
    DLoad3,
    /// Load the value at index 0 in the local variable array of the current
    /// frame into the stack.
    ALoad0,
    /// Load the value at index 1 in the local variable array of the current
    /// frame into the stack.
    ALoad1,
    /// Load the value at index 2 in the local variable array of the current
    /// frame into the stack.
    ALoad2,
    /// Load the value at index 3 in the local variable array of the current
    /// frame into the stack.
    ALoad3,
    IALoad,
    LALoad,
    FALoad,
    DALoad,
    /// Load `reference` from an array, the top tweo values on the stack are
    /// the `index` and `reference`. The loaded value is pushed back into the
    /// stack.
    AALoad,
    BALoad,
    CALoad,
    SALoad,
    /// Store `int` from the local variables array of the current frame
    /// and push it into the stack, the index is given as operand.
    IStore,
    /// Store `long` from the local variables array of the current frame
    /// and push it into the stack, the index is given as operand.
    LStore,
    /// Store `float` from the local variables array of the current frame
    /// and push it into the stack, the index is given as operand.
    FStore,
    /// Store `double` from the local variables array of the current frame
    /// and push it into the stack, the index is given as operand.
    DStore,
    /// Store `reference` into a local variable.
    AStore,
    /// Store `int` at index 0 in the local variables array of the current
    /// frame into the stack.
    IStore0,
    /// Store `int` at index 1 in the local variables array of the current
    /// frame into the stack.
    IStore1,
    /// Store `int` at index 2 in the local variables array of the current
    /// frame into the stack.
    IStore2,
    /// Store `int` at index 3 in the local variables array of the current
    /// frame into the stack.
    IStore3,
    /// Store `long` at index 0 in the local variables array of the current
    /// frame into the stack.
    LStore0,
    /// Store `long` at index 1 in the local variables array of the current
    /// frame into the stack.
    LStore1,
    /// Store `long` at index 2 in the local variables array of the current
    /// frame into the stack.
    LStore2,
    /// Store `long` at index 3 in the local variables array of the current
    /// frame into the stack.
    LStore3,
    /// Store `float` at index 0 in the local variables array of the current
    /// frame into the stack.
    FStore0,
    /// Store `float` at index 1 in the local variables array of the current
    /// frame into the stack.
    FStore1,
    /// Store `float` at index 2 in the local variables array of the current
    /// frame into the stack.
    FStore2,
    /// Store `float` at index 3 in the local variables array of the current
    /// frame into the stack.
    FStore3,
    /// Store `double` at index 0 in the local variables array of the current
    /// frame into the stack.
    DStore0,
    /// Store `double` at index 1 in the local variables array of the current
    /// frame into the stack.
    DStore1,
    /// Store `double` at index 2 in the local variables array of the current
    /// frame into the stack.
    DStore2,
    /// Store `double` at index 3 in the local variables array of the current
    /// frame into the stack.
    DStore3,
    AStore0,
    AStore1,
    AStore2,
    AStore3,
    IAStore,
    LAStore,
    FAStore,
    DAStore,
    /// Store into a `reference` array, the top three values on the stack are
    /// the value, index and reference to the array.
    AAStore,
    BAStore,
    CAStore,
    SAStore,
    Pop,
    Pop2,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    Swap,
    /// Pop the top two value from the stack (they must be of type `int`) then
    /// push their sum into the stack.
    IAdd,
    /// Pop the top two value from the stack (they must be of type `long`) then
    /// push their sum into the stack.
    LAdd,
    /// Pop the top two value from the stack (they must be of type `float`) then
    /// push their sum into the stack.
    FAdd,
    /// Pop the top two value from the stack (they must be of type `double`) then
    /// push their sum into the stack.
    DAdd,
    /// Pop the top two value from the stack (they must be of type `int`) then
    /// push their difference into the stack. The result is `value1` - `value2`
    /// and the values are laid as [`value1`, `value2`].
    ISub,
    /// Pop the top two value from the stack (they must be of type `long`) then
    /// push their difference into the stack. The result is `value1` - `value2`
    /// and the values are laid as [`value1`, `value2`].
    LSub,
    /// Pop the top two value from the stack (they must be of type `float`) then
    /// push their difference into the stack. The result is `value1` - `value2`
    /// and the values are laid as [`value1`, `value2`].
    FSub,
    /// Pop the top two value from the stack (they must be of type `double`) then
    /// push their difference into the stack. The result is `value1` - `value2`
    /// and the values are laid as [`value1`, `value2`].
    DSub,
    /// Pop the top two value from the stack (they must be of type `int`) then
    /// push their product into the stack. The result is `value1` * `value2`
    /// and the values are laid as [`value1`, `value2`].
    IMul,
    /// Pop the top two value from the stack (they must be of type `long`) then
    /// push their product into the stack. The result is `value1` * `value2`
    /// and the values are laid as [`value1`, `value2`].
    LMul,
    /// Pop the top two value from the stack (they must be of type `float`) then
    /// push their product into the stack. The result is `value1` * `value2`
    /// and the values are laid as [`value1`, `value2`].
    FMul,
    /// Pop the top two value from the stack (they must be of type `double`) then
    /// push their product into the stack. The result is `value1` * `value2`
    /// and the values are laid as [`value1`, `value2`].
    DMul,
    /// Pop the top two value from the stack (they must be of type `int`) then
    /// push their division into the stack. The result is `value1` / `value2`
    /// and the values are laid as [`value1`, `value2`].
    IDiv,
    /// Pop the top two value from the stack (they must be of type `long`) then
    /// push their division into the stack. The result is `value1` / `value2`
    /// and the values are laid as [`value1`, `value2`].
    LDiv,
    /// Pop the top two value from the stack (they must be of type `float`) then
    /// push their division into the stack. The result is `value1` / `value2`
    /// and the values are laid as [`value1`, `value2`].
    FDiv,
    /// Pop the top two value from the stack (they must be of type `double`) then
    /// push their division into the stack. The result is `value1` / `value2`
    /// and the values are laid as [`value1`, `value2`].
    DDiv,
    /// Pop the top two value from the stack (they must be of type `int`) then
    /// push their modulo into the stack. The result is `value1` / `value2`
    /// and the values are laid as [`value1`, `value2`].
    IRem,
    /// Pop the top two value from the stack (they must be of type `long`) then
    /// push their modulo into the stack. The result is `value1` / `value2`
    /// and the values are laid as [`value1`, `value2`].
    LRem,
    /// Pop the top two value from the stack (they must be of type `float`) then
    /// push their modulo into the stack. The result is `value1` / `value2`
    /// and the values are laid as [`value1`, `value2`].
    FRem,
    /// Pop the top two value from the stack (they must be of type `double`) then
    /// push their modulo into the stack. The result is `value1` / `value2`
    /// and the values are laid as [`value1`, `value2`].
    DRem,
    INeg,
    LNeg,
    FNeg,
    DNeg,
    IShl,
    LShl,
    IShr,
    LShr,
    IUShr,
    LUShr,
    Iand,
    Land,
    IOr,
    LOr,
    IXor,
    LXor,
    /// Increment the value in the local variables array stored at `index` given
    /// as an operand by the constant `const` given as an operand.
    IInc,
    I2L,
    I2F,
    I2D,
    L2I,
    L2F,
    L2D,
    F2I,
    F2L,
    F2D,
    D2I,
    D2L,
    D2F,
    I2B,
    I2C,
    I2S,
    LCmp,
    FCmpL,
    FCmpG,
    DCmpL,
    DCmpG,
    /// Branch to the target offset (given as operand) if the comparison is
    /// true, the compared values are the top value on the stack and 0.
    ///
    /// [value1] --->
    ///
    /// The value must be an `int` and the comparison is signed.
    ///
    /// Branch if `value` is equal to zero.
    IfEq,
    /// Branch if `value` is not equal to zero.
    IfNe,
    /// Branch if `value` is less than zero.
    IfLt,
    /// Branch if `value` is greater than or equal to zero.
    IfGe,
    /// Branch if `value` is greater than zero.
    IfGt,
    /// Branch if `value` is less than or equal to zero.
    IfLe,
    /// Branch to the target offset (given as operand) if the comparison is
    /// true, the compared values are the top two values in the stack laid
    /// out as (the values are interpreted as `int`). :
    ///
    /// [value1, value2] --->
    ///
    /// All comparisons are signed.
    ///
    /// Branch if the two top values on the stack are equal.
    IfICmpEq,
    /// Branch if the two top values on the stack are not equal.
    IfICmpNe,
    /// Branch if the `value1` is less than `value2`.
    IfICmpLt,
    /// Branch if `value1` is greater or equal than `value2`.
    IfICmpGe,
    /// Branch if `value1` is greater than `value2`.
    IfICmpGt,
    /// Branch if `value1` is less then or equal `value2`.
    IfICmpLe,
    IfACmpEq,
    IfACmpNe,
    /// Branch to the relative offset given as two 1 byte operands, execution
    /// continues at the relative offset from the address of the opcode of the
    /// goto instruction. The target address must be that of an opcode of an
    /// instruction within the method that contains this `goto` instruction.
    Goto,
    Jsr,
    Ret,
    TableSwitch,
    LookupSwitch,
    IReturn,
    LReturn,
    FReturn,
    DReturn,
    // REeturn `reference` from method.
    AReturn,
    Return,
    GetStatic,
    PutStatic,
    GetField,
    PutField,
    InvokeVirtual,
    InvokeSpecial,
    InvokeStatic,
    InvokeInterface,
    InvokeDynamic,
    New,
    NewArray,
    ANewArray,
    /// Pops the `reference` to the array from the stack and push its length
    /// into the stack.
    ArrayLength,
    AThrow,
    CheckCast,
    InstanceOf,
    MonitorEnter,
    MonitorExit,
    Wide,
    MultiANewArray,
    IfNull,
    IfNonNull,
    /// Similar to `goto` but the offset is given as a 4 byte value constructed
    /// from 4 1-byte operands. The constructed target address must be that of
    /// an opcode of an instruction within the method that contains the current
    /// `goto_w` instruction.
    GotoW,
    JsrW,
    Breakpoint,
    // Proxy value to signal unknown opcode values.
    Unspecified,
}

impl fmt::Display for OPCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nop => write!(f, "nop"),
            Self::AConstNull => write!(f, "aconst_null"),
            Self::IconstM1 => write!(f, "iconst_m1"),
            Self::Iconst0 => write!(f, "iconst_0"),
            Self::Iconst1 => write!(f, "iconst_1"),
            Self::Iconst2 => write!(f, "iconst_2"),
            Self::Iconst3 => write!(f, "iconst_3"),
            Self::Iconst4 => write!(f, "iconst_4"),
            Self::Iconst5 => write!(f, "iconst_5"),
            Self::Lconst0 => write!(f, "lconst_0"),
            Self::Lconst1 => write!(f, "lconst_1"),
            Self::Fconst0 => write!(f, "fconst_0"),
            Self::Fconst1 => write!(f, "fconst_1"),
            Self::Fconst2 => write!(f, "fconst_2"),
            Self::Dconst0 => write!(f, "dconst_0"),
            Self::Dconst1 => write!(f, "dconst_1"),
            Self::BiPush => write!(f, "bipush"),
            Self::SiPush => write!(f, "sipush"),
            Self::Ldc => write!(f, "ldc"),
            Self::LdcW => write!(f, "ldc_w"),
            Self::Ldc2W => write!(f, "ldc2_w"),
            Self::ILoad => write!(f, "iload"),
            Self::LLoad => write!(f, "lload"),
            Self::FLoad => write!(f, "fload"),
            Self::DLoad => write!(f, "dload"),
            Self::ALoad => write!(f, "aload"),
            Self::ILoad0 => write!(f, "iload_0"),
            Self::ILoad1 => write!(f, "iload_1"),
            Self::ILoad2 => write!(f, "iload_2"),
            Self::ILoad3 => write!(f, "iload_3"),
            Self::LLoad0 => write!(f, "lload_0"),
            Self::LLoad1 => write!(f, "lload_1"),
            Self::LLoad2 => write!(f, "lload_2"),
            Self::LLoad3 => write!(f, "lload_3"),
            Self::FLoad0 => write!(f, "fload_0"),
            Self::FLoad1 => write!(f, "fload_1"),
            Self::FLoad2 => write!(f, "fload_2"),
            Self::FLoad3 => write!(f, "fload_3"),
            Self::DLoad0 => write!(f, "dload_0"),
            Self::DLoad1 => write!(f, "dload_1"),
            Self::DLoad2 => write!(f, "dload_2"),
            Self::DLoad3 => write!(f, "dload_3"),
            Self::ALoad0 => write!(f, "aload_0"),
            Self::ALoad1 => write!(f, "aload_1"),
            Self::ALoad2 => write!(f, "aload_2"),
            Self::ALoad3 => write!(f, "aload_3"),
            Self::IALoad => write!(f, "iaload"),
            Self::LALoad => write!(f, "laload"),
            Self::FALoad => write!(f, "faload"),
            Self::DALoad => write!(f, "daload"),
            Self::AALoad => write!(f, "aaload"),
            Self::BALoad => write!(f, "baload"),
            Self::CALoad => write!(f, "caload"),
            Self::SALoad => write!(f, "saload"),
            Self::IStore => write!(f, "istore"),
            Self::LStore => write!(f, "lstore"),
            Self::FStore => write!(f, "fstore"),
            Self::DStore => write!(f, "dstore"),
            Self::AStore => write!(f, "astore"),
            Self::IStore0 => write!(f, "istore_0"),
            Self::IStore1 => write!(f, "istore_1"),
            Self::IStore2 => write!(f, "istore_2"),
            Self::IStore3 => write!(f, "istore_3"),
            Self::LStore0 => write!(f, "lstore_0"),
            Self::LStore1 => write!(f, "lstore_1"),
            Self::LStore2 => write!(f, "lstore_2"),
            Self::LStore3 => write!(f, "lstore_3"),
            Self::FStore0 => write!(f, "fstore_0"),
            Self::FStore1 => write!(f, "fstore_1"),
            Self::FStore2 => write!(f, "fstore_2"),
            Self::FStore3 => write!(f, "fstore_3"),
            Self::DStore0 => write!(f, "dstore_0"),
            Self::DStore1 => write!(f, "dstore_1"),
            Self::DStore2 => write!(f, "dstore_2"),
            Self::DStore3 => write!(f, "dstore_3"),
            Self::AStore0 => write!(f, "astore_0"),
            Self::AStore1 => write!(f, "astore_1"),
            Self::AStore2 => write!(f, "astore_2"),
            Self::AStore3 => write!(f, "astore_3"),
            Self::IAStore => write!(f, "iastore"),
            Self::LAStore => write!(f, "lastore"),
            Self::FAStore => write!(f, "fastore"),
            Self::DAStore => write!(f, "dastore"),
            Self::AAStore => write!(f, "aastore"),
            Self::BAStore => write!(f, "bastore"),
            Self::CAStore => write!(f, "castore"),
            Self::SAStore => write!(f, "sastore"),
            Self::Pop => write!(f, "pop"),
            Self::Pop2 => write!(f, "pop_2"),
            Self::Dup => write!(f, "dup"),
            Self::DupX1 => write!(f, "dup_x1"),
            Self::DupX2 => write!(f, "dup_x2"),
            Self::Dup2 => write!(f, "dup2"),
            Self::Dup2X1 => write!(f, "dup2_x1"),
            Self::Dup2X2 => write!(f, "dup2_x2"),
            Self::Swap => write!(f, "swap"),
            Self::IAdd => write!(f, "iadd"),
            Self::LAdd => write!(f, "ladd"),
            Self::FAdd => write!(f, "fadd"),
            Self::DAdd => write!(f, "dadd"),
            Self::ISub => write!(f, "isub"),
            Self::LSub => write!(f, "lsub"),
            Self::FSub => write!(f, "fsub"),
            Self::DSub => write!(f, "dsub"),
            Self::IMul => write!(f, "imul"),
            Self::LMul => write!(f, "lmul"),
            Self::FMul => write!(f, "fmul"),
            Self::DMul => write!(f, "dmul"),
            Self::IDiv => write!(f, "idiv"),
            Self::LDiv => write!(f, "ldiv"),
            Self::FDiv => write!(f, "fdiv"),
            Self::DDiv => write!(f, "ddiv"),
            Self::IRem => write!(f, "irem"),
            Self::LRem => write!(f, "lrem"),
            Self::FRem => write!(f, "frem"),
            Self::DRem => write!(f, "drem"),
            Self::INeg => write!(f, "ineg"),
            Self::LNeg => write!(f, "lneg"),
            Self::FNeg => write!(f, "fneg"),
            Self::DNeg => write!(f, "dneg"),
            Self::IShl => write!(f, "ishl"),
            Self::LShl => write!(f, "lshl"),
            Self::IShr => write!(f, "ishr"),
            Self::LShr => write!(f, "lshr"),
            Self::IUShr => write!(f, "iushr"),
            Self::LUShr => write!(f, "lushr"),
            Self::Iand => write!(f, "iand"),
            Self::Land => write!(f, "land"),
            Self::IOr => write!(f, "ior"),
            Self::LOr => write!(f, "lor"),
            Self::IXor => write!(f, "ixor"),
            Self::LXor => write!(f, "lxor"),
            Self::IInc => write!(f, "iinc"),
            Self::I2L => write!(f, "i2l"),
            Self::I2F => write!(f, "i2f"),
            Self::I2D => write!(f, "i2d"),
            Self::L2I => write!(f, "l2i"),
            Self::L2F => write!(f, "l2f"),
            Self::L2D => write!(f, "l2d"),
            Self::F2I => write!(f, "f2i"),
            Self::F2L => write!(f, "f2l"),
            Self::F2D => write!(f, "f2d"),
            Self::D2I => write!(f, "d2i"),
            Self::D2L => write!(f, "d2l"),
            Self::D2F => write!(f, "d2f"),
            Self::I2B => write!(f, "i2b"),
            Self::I2C => write!(f, "i2c"),
            Self::I2S => write!(f, "i2s"),
            Self::LCmp => write!(f, "lcmp"),
            Self::FCmpL => write!(f, "fcmpl"),
            Self::FCmpG => write!(f, "fcmpg"),
            Self::DCmpL => write!(f, "dcmpl"),
            Self::DCmpG => write!(f, "dcmpg"),
            Self::IfEq => write!(f, "ifeq"),
            Self::IfNe => write!(f, "ifne"),
            Self::IfLt => write!(f, "iflt"),
            Self::IfGe => write!(f, "ifge"),
            Self::IfGt => write!(f, "ifgt"),
            Self::IfLe => write!(f, "ifle"),
            Self::IfICmpEq => write!(f, "if_icmpeq"),
            Self::IfICmpNe => write!(f, "if_icmpne"),
            Self::IfICmpLt => write!(f, "if_icmplt"),
            Self::IfICmpGe => write!(f, "if_icmpge"),
            Self::IfICmpGt => write!(f, "if_icmpgt"),
            Self::IfICmpLe => write!(f, "if_icmple"),
            Self::IfACmpEq => write!(f, "if_acmpeq"),
            Self::IfACmpNe => write!(f, "if_acmpne"),
            Self::Goto => write!(f, "goto"),
            Self::Jsr => write!(f, "jsr"),
            Self::Ret => write!(f, "ret"),
            Self::TableSwitch => write!(f, "tableswitch"),
            Self::LookupSwitch => write!(f, "lookupswitch"),
            Self::IReturn => write!(f, "ireturn"),
            Self::LReturn => write!(f, "lreturn"),
            Self::FReturn => write!(f, "freturn"),
            Self::DReturn => write!(f, "dreturn"),
            Self::AReturn => write!(f, "areturn"),
            Self::Return => write!(f, "return"),
            Self::GetStatic => write!(f, "getstatic"),
            Self::PutStatic => write!(f, "putstatic"),
            Self::GetField => write!(f, "getfield"),
            Self::PutField => write!(f, "putfield"),
            Self::InvokeVirtual => write!(f, "invokevirtual"),
            Self::InvokeSpecial => write!(f, "invokespecial"),
            Self::InvokeStatic => write!(f, "invokestatic"),
            Self::InvokeInterface => write!(f, "invokeinterface"),
            Self::InvokeDynamic => write!(f, "invokedynamic"),
            Self::New => write!(f, "new"),
            Self::NewArray => write!(f, "newarray"),
            Self::ANewArray => write!(f, "anewarray"),
            Self::ArrayLength => write!(f, "arraylength"),
            Self::AThrow => write!(f, "athrow"),
            Self::CheckCast => write!(f, "checkcast"),
            Self::InstanceOf => write!(f, "instanceof"),
            Self::MonitorEnter => write!(f, "monitorenter"),
            Self::MonitorExit => write!(f, "monitorexit"),
            Self::Wide => write!(f, "wide"),
            Self::MultiANewArray => write!(f, "multianewarray"),
            Self::IfNull => write!(f, "ifnull"),
            Self::IfNonNull => write!(f, "ifnonnull"),
            Self::GotoW => write!(f, "goto_w"),
            Self::JsrW => write!(f, "jsr_w"),
            Self::Breakpoint => write!(f, "breakpoint"),
            _ => write!(f, "unspecified"),
        }
    }
}

// Since bytecode is initially loaded as `Vec<u8>` we need a way to convert it
// to `OPCode` enum, this might be done better with a macro but copy paste and
// move on for now.
impl From<u8> for OPCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::Nop,
            1 => Self::AConstNull,
            2 => Self::IconstM1,
            3 => Self::Iconst0,
            4 => Self::Iconst1,
            5 => Self::Iconst2,
            6 => Self::Iconst3,
            7 => Self::Iconst4,
            8 => Self::Iconst5,
            9 => Self::Lconst0,
            10 => Self::Lconst1,
            11 => Self::Fconst0,
            12 => Self::Fconst1,
            13 => Self::Fconst2,
            14 => Self::Dconst0,
            15 => Self::Dconst1,
            16 => Self::BiPush,
            17 => Self::SiPush,
            18 => Self::Ldc,
            19 => Self::LdcW,
            20 => Self::Ldc2W,
            21 => Self::ILoad,
            22 => Self::LLoad,
            23 => Self::FLoad,
            24 => Self::DLoad,
            25 => Self::ALoad,
            26 => Self::ILoad0,
            27 => Self::ILoad1,
            28 => Self::ILoad2,
            29 => Self::ILoad3,
            30 => Self::LLoad0,
            31 => Self::LLoad1,
            32 => Self::LLoad2,
            33 => Self::LLoad3,
            34 => Self::FLoad0,
            35 => Self::FLoad1,
            36 => Self::FLoad2,
            37 => Self::FLoad3,
            38 => Self::DLoad0,
            39 => Self::DLoad1,
            40 => Self::DLoad2,
            41 => Self::DLoad3,
            42 => Self::ALoad0,
            43 => Self::ALoad1,
            44 => Self::ALoad2,
            45 => Self::ALoad3,
            46 => Self::IALoad,
            47 => Self::LALoad,
            48 => Self::FALoad,
            49 => Self::DALoad,
            50 => Self::AALoad,
            51 => Self::BALoad,
            52 => Self::CALoad,
            53 => Self::SALoad,
            54 => Self::IStore,
            55 => Self::LStore,
            56 => Self::FStore,
            57 => Self::DStore,
            58 => Self::AStore,
            59 => Self::IStore0,
            60 => Self::IStore1,
            61 => Self::IStore2,
            62 => Self::IStore3,
            63 => Self::LStore0,
            64 => Self::LStore1,
            65 => Self::LStore2,
            66 => Self::LStore3,
            67 => Self::FStore0,
            68 => Self::FStore1,
            69 => Self::FStore2,
            70 => Self::FStore3,
            71 => Self::DStore0,
            72 => Self::DStore1,
            73 => Self::DStore2,
            74 => Self::DStore3,
            75 => Self::AStore0,
            76 => Self::AStore1,
            77 => Self::AStore2,
            78 => Self::AStore3,
            79 => Self::IAStore,
            80 => Self::LAStore,
            81 => Self::FAStore,
            82 => Self::DAStore,
            83 => Self::AAStore,
            84 => Self::BAStore,
            85 => Self::CAStore,
            86 => Self::SAStore,
            87 => Self::Pop,
            88 => Self::Pop2,
            89 => Self::Dup,
            90 => Self::DupX1,
            91 => Self::DupX2,
            92 => Self::Dup2,
            93 => Self::Dup2X1,
            94 => Self::Dup2X2,
            95 => Self::Swap,
            96 => Self::IAdd,
            97 => Self::LAdd,
            98 => Self::FAdd,
            99 => Self::DAdd,
            100 => Self::ISub,
            101 => Self::LSub,
            102 => Self::FSub,
            103 => Self::DSub,
            104 => Self::IMul,
            105 => Self::LMul,
            106 => Self::FMul,
            107 => Self::DMul,
            108 => Self::IDiv,
            109 => Self::LDiv,
            110 => Self::FDiv,
            111 => Self::DDiv,
            112 => Self::IRem,
            113 => Self::LRem,
            114 => Self::FRem,
            115 => Self::DRem,
            116 => Self::INeg,
            117 => Self::LNeg,
            118 => Self::FNeg,
            119 => Self::DNeg,
            120 => Self::IShl,
            121 => Self::LShl,
            122 => Self::IShr,
            123 => Self::LShr,
            124 => Self::IUShr,
            125 => Self::LUShr,
            126 => Self::Iand,
            127 => Self::Land,
            128 => Self::IOr,
            129 => Self::LOr,
            130 => Self::IXor,
            131 => Self::LXor,
            132 => Self::IInc,
            133 => Self::I2L,
            134 => Self::I2F,
            135 => Self::I2D,
            136 => Self::L2I,
            137 => Self::L2F,
            138 => Self::L2D,
            139 => Self::F2I,
            140 => Self::F2L,
            141 => Self::F2D,
            142 => Self::D2I,
            143 => Self::D2L,
            144 => Self::D2F,
            145 => Self::I2B,
            146 => Self::I2C,
            147 => Self::I2S,
            148 => Self::LCmp,
            149 => Self::FCmpL,
            150 => Self::FCmpG,
            151 => Self::DCmpL,
            152 => Self::DCmpG,
            153 => Self::IfEq,
            154 => Self::IfNe,
            155 => Self::IfLt,
            156 => Self::IfGe,
            157 => Self::IfGt,
            158 => Self::IfLe,
            159 => Self::IfICmpEq,
            160 => Self::IfICmpNe,
            161 => Self::IfICmpLt,
            162 => Self::IfICmpGe,
            163 => Self::IfICmpGt,
            164 => Self::IfICmpLe,
            165 => Self::IfACmpEq,
            166 => Self::IfACmpNe,
            167 => Self::Goto,
            168 => Self::Jsr,
            169 => Self::Ret,
            170 => Self::TableSwitch,
            171 => Self::LookupSwitch,
            172 => Self::IReturn,
            173 => Self::LReturn,
            174 => Self::FReturn,
            175 => Self::DReturn,
            176 => Self::AReturn,
            177 => Self::Return,
            178 => Self::GetStatic,
            179 => Self::PutStatic,
            180 => Self::GetField,
            181 => Self::PutField,
            182 => Self::InvokeVirtual,
            183 => Self::InvokeSpecial,
            184 => Self::InvokeStatic,
            185 => Self::InvokeInterface,
            186 => Self::InvokeDynamic,
            187 => Self::New,
            188 => Self::NewArray,
            189 => Self::ANewArray,
            190 => Self::ArrayLength,
            191 => Self::AThrow,
            192 => Self::CheckCast,
            193 => Self::InstanceOf,
            194 => Self::MonitorEnter,
            195 => Self::MonitorExit,
            196 => Self::Wide,
            197 => Self::MultiANewArray,
            198 => Self::IfNull,
            199 => Self::IfNonNull,
            200 => Self::GotoW,
            201 => Self::JsrW,
            202 => Self::Breakpoint,
            203..=u8::MAX => Self::Unspecified,
        }
    }
}
