#[non_exhaustive]
pub struct Opcodes;

pub struct Opcode {
    pub opcode: u8,
    pub opname: &'static str,
    pub oplen: u8,
}

#[allow(unused)]
impl Opcodes {
    pub const AALOAD: Opcode = Opcode { opcode: 0x32, opname: "aaload", oplen: 1 };
    pub const AASTORE: Opcode = Opcode { opcode: 0x53, opname: "aastore", oplen: 1 };
    pub const ACONST_NULL: Opcode = Opcode { opcode: 0x01, opname: "aconst_null", oplen: 1 };
    pub const ALOAD: Opcode = Opcode { opcode: 0x19, opname: "aload", oplen: 2 };
    pub const ALOAD_0: Opcode = Opcode { opcode: 0x2a, opname: "aload_0", oplen: 1 };
    pub const ALOAD_1: Opcode = Opcode { opcode: 0x2b, opname: "aload_1", oplen: 1 };
    pub const ALOAD_2: Opcode = Opcode { opcode: 0x2c, opname: "aload_2", oplen: 1 };
    pub const ALOAD_3: Opcode = Opcode { opcode: 0x2d, opname: "aload_3", oplen: 1 };
    pub const ANEWARRAY: Opcode = Opcode { opcode: 0xbd, opname: "anewarray", oplen: 3 };
    pub const ARETURN: Opcode = Opcode { opcode: 0xb0, opname: "areturn", oplen: 1 };
    pub const ARRAYLENGTH: Opcode = Opcode { opcode: 0xbe, opname: "arraylength", oplen: 1 };
    pub const ASTORE: Opcode = Opcode { opcode: 0x3a, opname: "astore", oplen: 2 };
    pub const ASTORE_0: Opcode = Opcode { opcode: 0x4b, opname: "astore_0", oplen: 1 };
    pub const ASTORE_1: Opcode = Opcode { opcode: 0x4c, opname: "astore_1", oplen: 1 };
    pub const ASTORE_2: Opcode = Opcode { opcode: 0x4d, opname: "astore_2", oplen: 1 };
    pub const ASTORE_3: Opcode = Opcode { opcode: 0x4e, opname: "astore_3", oplen: 1 };
    pub const ATHROW: Opcode = Opcode { opcode: 0xbf, opname: "athrow", oplen: 1 };
    pub const BALOAD: Opcode = Opcode { opcode: 0x33, opname: "baload", oplen: 1 };
    pub const BASTORE: Opcode = Opcode { opcode: 0x54, opname: "bastore", oplen: 1 };
    pub const BIPUSH: Opcode = Opcode { opcode: 0x10, opname: "bipush", oplen: 2 };
    pub const CALOAD: Opcode = Opcode { opcode: 0x34, opname: "caload", oplen: 1 };
    pub const CASTORE: Opcode = Opcode { opcode: 0x55, opname: "castore", oplen: 1 };
    pub const CHECKCAST: Opcode = Opcode { opcode: 0xc0, opname: "checkcast", oplen: 3 };
    pub const D2F: Opcode = Opcode { opcode: 0x90, opname: "d2f", oplen: 1 };
    pub const D2I: Opcode = Opcode { opcode: 0x8e, opname: "d2i", oplen: 1 };
    pub const D2L: Opcode = Opcode { opcode: 0x8f, opname: "d2l", oplen: 1 };
    pub const DADD: Opcode = Opcode { opcode: 0x63, opname: "dadd", oplen: 1 };
    pub const DALOAD: Opcode = Opcode { opcode: 0x31, opname: "daload", oplen: 1 };
    pub const DASTORE: Opcode = Opcode { opcode: 0x52, opname: "dastore", oplen: 1 };
    pub const DCMPG: Opcode = Opcode { opcode: 0x98, opname: "dcmpg", oplen: 1 };
    pub const DCMPL: Opcode = Opcode { opcode: 0x97, opname: "dcmpl", oplen: 1 };
    pub const DCONST_0: Opcode = Opcode { opcode: 0x0e, opname: "dconst_0", oplen: 1 };
    pub const DCONST_1: Opcode = Opcode { opcode: 0x0f, opname: "dconst_1", oplen: 1 };
    pub const DDIV: Opcode = Opcode { opcode: 0x6f, opname: "ddiv", oplen: 1 };
    pub const DLOAD: Opcode = Opcode { opcode: 0x18, opname: "dload", oplen: 2 };
    pub const DLOAD_0: Opcode = Opcode { opcode: 0x26, opname: "dload_0", oplen: 1 };
    pub const DLOAD_1: Opcode = Opcode { opcode: 0x27, opname: "dload_1", oplen: 1 };
    pub const DLOAD_2: Opcode = Opcode { opcode: 0x28, opname: "dload_2", oplen: 1 };
    pub const DLOAD_3: Opcode = Opcode { opcode: 0x29, opname: "dload_3", oplen: 1 };
    pub const DMUL: Opcode = Opcode { opcode: 0x6b, opname: "dmul", oplen: 1 };
    pub const DNEG: Opcode = Opcode { opcode: 0x77, opname: "dneg", oplen: 1 };
    pub const DREM: Opcode = Opcode { opcode: 0x73, opname: "drem", oplen: 1 };
    pub const DRETURN: Opcode = Opcode { opcode: 0xaf, opname: "dreturn", oplen: 1 };
    pub const DSTORE: Opcode = Opcode { opcode: 0x39, opname: "dstore", oplen: 2 };
    pub const DSTORE_0: Opcode = Opcode { opcode: 0x47, opname: "dstore_0", oplen: 1 };
    pub const DSTORE_1: Opcode = Opcode { opcode: 0x48, opname: "dstore_1", oplen: 1 };
    pub const DSTORE_2: Opcode = Opcode { opcode: 0x49, opname: "dstore_2", oplen: 1 };
    pub const DSTORE_3: Opcode = Opcode { opcode: 0x4a, opname: "dstore_3", oplen: 1 };
    pub const DSUB: Opcode = Opcode { opcode: 0x67, opname: "dsub", oplen: 1 };
    pub const DUP: Opcode = Opcode { opcode: 0x59, opname: "dup", oplen: 1 };
    pub const DUP2: Opcode = Opcode { opcode: 0x5c, opname: "dup2", oplen: 1 };
    pub const DUP2_X1: Opcode = Opcode { opcode: 0x5d, opname: "dup2_x1", oplen: 1 };
    pub const DUP2_X2: Opcode = Opcode { opcode: 0x5e, opname: "dup2_x2", oplen: 1 };
    pub const DUP_X1: Opcode = Opcode { opcode: 0x5a, opname: "dup_x1", oplen: 1 };
    pub const DUP_X2: Opcode = Opcode { opcode: 0x5b, opname: "dup_x2", oplen: 1 };
    pub const F2D: Opcode = Opcode { opcode: 0x8d, opname: "f2d", oplen: 1 };
    pub const F2I: Opcode = Opcode { opcode: 0x8b, opname: "f2i", oplen: 1 };
    pub const F2L: Opcode = Opcode { opcode: 0x8c, opname: "f2l", oplen: 1 };
    pub const FADD: Opcode = Opcode { opcode: 0x62, opname: "fadd", oplen: 1 };
    pub const FALOAD: Opcode = Opcode { opcode: 0x30, opname: "faload", oplen: 1 };
    pub const FASTORE: Opcode = Opcode { opcode: 0x51, opname: "fastore", oplen: 1 };
    pub const FCMPG: Opcode = Opcode { opcode: 0x96, opname: "fcmpg", oplen: 1 };
    pub const FCMPL: Opcode = Opcode { opcode: 0x95, opname: "fcmpl", oplen: 1 };
    pub const FCONST_0: Opcode = Opcode { opcode: 0x0b, opname: "fconst_0", oplen: 1 };
    pub const FCONST_1: Opcode = Opcode { opcode: 0x0c, opname: "fconst_1", oplen: 1 };
    pub const FCONST_2: Opcode = Opcode { opcode: 0x0d, opname: "fconst_2", oplen: 1 };
    pub const FDIV: Opcode = Opcode { opcode: 0x6e, opname: "fdiv", oplen: 1 };
    pub const FLOAD: Opcode = Opcode { opcode: 0x17, opname: "fload", oplen: 2 };
    pub const FLOAD_0: Opcode = Opcode { opcode: 0x22, opname: "fload_0", oplen: 1 };
    pub const FLOAD_1: Opcode = Opcode { opcode: 0x23, opname: "fload_1", oplen: 1 };
    pub const FLOAD_2: Opcode = Opcode { opcode: 0x24, opname: "fload_2", oplen: 1 };
    pub const FLOAD_3: Opcode = Opcode { opcode: 0x25, opname: "fload_3", oplen: 1 };
    pub const FMUL: Opcode = Opcode { opcode: 0x6a, opname: "fmul", oplen: 1 };
    pub const FNEG: Opcode = Opcode { opcode: 0x76, opname: "fneg", oplen: 1 };
    pub const FREM: Opcode = Opcode { opcode: 0x72, opname: "frem", oplen: 1 };
    pub const FRETURN: Opcode = Opcode { opcode: 0xae, opname: "freturn", oplen: 1 };
    pub const FSTORE: Opcode = Opcode { opcode: 0x38, opname: "fstore", oplen: 2 };
    pub const FSTORE_0: Opcode = Opcode { opcode: 0x43, opname: "fstore_0", oplen: 1 };
    pub const FSTORE_1: Opcode = Opcode { opcode: 0x44, opname: "fstore_1", oplen: 1 };
    pub const FSTORE_2: Opcode = Opcode { opcode: 0x45, opname: "fstore_2", oplen: 1 };
    pub const FSTORE_3: Opcode = Opcode { opcode: 0x46, opname: "fstore_3", oplen: 1 };
    pub const FSUB: Opcode = Opcode { opcode: 0x66, opname: "fsub", oplen: 1 };
    pub const GETFIELD: Opcode = Opcode { opcode: 0xb4, opname: "getfield", oplen: 3 };
    pub const GETSTATIC: Opcode = Opcode { opcode: 0xb2, opname: "getstatic", oplen: 3 };
    pub const GOTO: Opcode = Opcode { opcode: 0xa7, opname: "goto", oplen: 3 };
    pub const GOTO_W: Opcode = Opcode { opcode: 0xc8, opname: "goto_w", oplen: 5 };
    pub const I2B: Opcode = Opcode { opcode: 0x91, opname: "i2b", oplen: 1 };
    pub const I2C: Opcode = Opcode { opcode: 0x92, opname: "i2c", oplen: 1 };
    pub const I2D: Opcode = Opcode { opcode: 0x87, opname: "i2d", oplen: 1 };
    pub const I2F: Opcode = Opcode { opcode: 0x86, opname: "i2f", oplen: 1 };
    pub const I2L: Opcode = Opcode { opcode: 0x85, opname: "i2l", oplen: 1 };
    pub const I2S: Opcode = Opcode { opcode: 0x93, opname: "i2s", oplen: 1 };
    pub const IADD: Opcode = Opcode { opcode: 0x60, opname: "iadd", oplen: 1 };
    pub const IALOAD: Opcode = Opcode { opcode: 0x2e, opname: "iaload", oplen: 1 };
    pub const IAND: Opcode = Opcode { opcode: 0x7e, opname: "iand", oplen: 1 };
    pub const IASTORE: Opcode = Opcode { opcode: 0x4f, opname: "iastore", oplen: 1 };
    pub const ICONST_0: Opcode = Opcode { opcode: 0x03, opname: "iconst_0", oplen: 1 };
    pub const ICONST_1: Opcode = Opcode { opcode: 0x04, opname: "iconst_1", oplen: 1 };
    pub const ICONST_2: Opcode = Opcode { opcode: 0x05, opname: "iconst_2", oplen: 1 };
    pub const ICONST_3: Opcode = Opcode { opcode: 0x06, opname: "iconst_3", oplen: 1 };
    pub const ICONST_4: Opcode = Opcode { opcode: 0x07, opname: "iconst_4", oplen: 1 };
    pub const ICONST_5: Opcode = Opcode { opcode: 0x08, opname: "iconst_5", oplen: 1 };
    pub const ICONST_M1: Opcode = Opcode { opcode: 0x02, opname: "iconst_m1", oplen: 1 };
    pub const IDIV: Opcode = Opcode { opcode: 0x6c, opname: "idiv", oplen: 1 };
    pub const IF_ACMPEQ: Opcode = Opcode { opcode: 0xa5, opname: "if_acmpeq", oplen: 3 };
    pub const IF_ACMPNE: Opcode = Opcode { opcode: 0xa6, opname: "if_acmpne", oplen: 3 };
    pub const IF_ICMPEQ: Opcode = Opcode { opcode: 0x9f, opname: "if_icmpeq", oplen: 3 };
    pub const IF_ICMPGE: Opcode = Opcode { opcode: 0xa2, opname: "if_icmpge", oplen: 3 };
    pub const IF_ICMPGT: Opcode = Opcode { opcode: 0xa3, opname: "if_icmpgt", oplen: 3 };
    pub const IF_ICMPLE: Opcode = Opcode { opcode: 0xa4, opname: "if_icmple", oplen: 3 };
    pub const IF_ICMPLT: Opcode = Opcode { opcode: 0xa1, opname: "if_icmplt", oplen: 3 };
    pub const IF_ICMPNE: Opcode = Opcode { opcode: 0xa0, opname: "if_icmpne", oplen: 3 };
    pub const IFEQ: Opcode = Opcode { opcode: 0x99, opname: "ifeq", oplen: 3 };
    pub const IFGE: Opcode = Opcode { opcode: 0x9c, opname: "ifge", oplen: 3 };
    pub const IFGT: Opcode = Opcode { opcode: 0x9d, opname: "ifgt", oplen: 3 };
    pub const IFLE: Opcode = Opcode { opcode: 0x9e, opname: "ifle", oplen: 3 };
    pub const IFLT: Opcode = Opcode { opcode: 0x9b, opname: "iflt", oplen: 3 };
    pub const IFNE: Opcode = Opcode { opcode: 0x9a, opname: "ifne", oplen: 3 };
    pub const IFNONNULL: Opcode = Opcode { opcode: 0xc7, opname: "ifnonnull", oplen: 3 };
    pub const IFNULL: Opcode = Opcode { opcode: 0xc6, opname: "ifnull", oplen: 3 };
    pub const IINC: Opcode = Opcode { opcode: 0x84, opname: "iinc", oplen: 3 };
    pub const ILOAD: Opcode = Opcode { opcode: 0x15, opname: "iload", oplen: 2 };
    pub const ILOAD_0: Opcode = Opcode { opcode: 0x1a, opname: "iload_0", oplen: 1 };
    pub const ILOAD_1: Opcode = Opcode { opcode: 0x1b, opname: "iload_1", oplen: 1 };
    pub const ILOAD_2: Opcode = Opcode { opcode: 0x1c, opname: "iload_2", oplen: 1 };
    pub const ILOAD_3: Opcode = Opcode { opcode: 0x1d, opname: "iload_3", oplen: 1 };
    pub const IMUL: Opcode = Opcode { opcode: 0x68, opname: "imul", oplen: 1 };
    pub const INEG: Opcode = Opcode { opcode: 0x74, opname: "ineg", oplen: 1 };
    pub const INSTANCEOF: Opcode = Opcode { opcode: 0xc1, opname: "instanceof", oplen: 3 };
    pub const INVOKEDYNAMIC: Opcode = Opcode { opcode: 0xba, opname: "invokedynamic", oplen: 5 };
    pub const INVOKEINTERFACE: Opcode = Opcode { opcode: 0xb9, opname: "invokeinterface", oplen: 5 };
    pub const INVOKESPECIAL: Opcode = Opcode { opcode: 0xb7, opname: "invokespecial", oplen: 3 };
    pub const INVOKESTATIC: Opcode = Opcode { opcode: 0xb8, opname: "invokestatic", oplen: 3 };
    pub const INVOKEVIRTUAL: Opcode = Opcode { opcode: 0xb6, opname: "invokevirtual", oplen: 3 };
    pub const IOR: Opcode = Opcode { opcode: 0x80, opname: "ior", oplen: 1 };
    pub const IREM: Opcode = Opcode { opcode: 0x70, opname: "irem", oplen: 1 };
    pub const IRETURN: Opcode = Opcode { opcode: 0xac, opname: "ireturn", oplen: 1 };
    pub const ISHL: Opcode = Opcode { opcode: 0x78, opname: "ishl", oplen: 1 };
    pub const ISHR: Opcode = Opcode { opcode: 0x7a, opname: "ishr", oplen: 1 };
    pub const ISTORE: Opcode = Opcode { opcode: 0x36, opname: "istore", oplen: 2 };
    pub const ISTORE_0: Opcode = Opcode { opcode: 0x3b, opname: "istore_0", oplen: 1 };
    pub const ISTORE_1: Opcode = Opcode { opcode: 0x3c, opname: "istore_1", oplen: 1 };
    pub const ISTORE_2: Opcode = Opcode { opcode: 0x3d, opname: "istore_2", oplen: 1 };
    pub const ISTORE_3: Opcode = Opcode { opcode: 0x3e, opname: "istore_3", oplen: 1 };
    pub const ISUB: Opcode = Opcode { opcode: 0x64, opname: "isub", oplen: 1 };
    pub const IUSHR: Opcode = Opcode { opcode: 0x7c, opname: "iushr", oplen: 1 };
    pub const IXOR: Opcode = Opcode { opcode: 0x82, opname: "ixor", oplen: 1 };
    pub const JSR: Opcode = Opcode { opcode: 0xa8, opname: "jsr", oplen: 3 };
    pub const JSR_W: Opcode = Opcode { opcode: 0xc9, opname: "jsr_w", oplen: 5 };
    pub const L2D: Opcode = Opcode { opcode: 0x8a, opname: "l2d", oplen: 1 };
    pub const L2F: Opcode = Opcode { opcode: 0x89, opname: "l2f", oplen: 1 };
    pub const L2I: Opcode = Opcode { opcode: 0x88, opname: "l2i", oplen: 1 };
    pub const LADD: Opcode = Opcode { opcode: 0x61, opname: "ladd", oplen: 1 };
    pub const LALOAD: Opcode = Opcode { opcode: 0x2f, opname: "laload", oplen: 1 };
    pub const LAND: Opcode = Opcode { opcode: 0x7f, opname: "land", oplen: 1 };
    pub const LASTORE: Opcode = Opcode { opcode: 0x50, opname: "lastore", oplen: 1 };
    pub const LCMP: Opcode = Opcode { opcode: 0x94, opname: "lcmp", oplen: 1 };
    pub const LCONST_0: Opcode = Opcode { opcode: 0x09, opname: "lconst_0", oplen: 1 };
    pub const LCONST_1: Opcode = Opcode { opcode: 0x0a, opname: "lconst_1", oplen: 1 };
    pub const LDC: Opcode = Opcode { opcode: 0x12, opname: "ldc", oplen: 2 };
    pub const LDC2_W: Opcode = Opcode { opcode: 0x14, opname: "ldc2_w", oplen: 3 };
    pub const LDC_W: Opcode = Opcode { opcode: 0x13, opname: "ldc_w", oplen: 3 };
    pub const LDIV: Opcode = Opcode { opcode: 0x6d, opname: "ldiv", oplen: 1 };
    pub const LLOAD: Opcode = Opcode { opcode: 0x16, opname: "lload", oplen: 2 };
    pub const LLOAD_0: Opcode = Opcode { opcode: 0x1e, opname: "lload_0", oplen: 1 };
    pub const LLOAD_1: Opcode = Opcode { opcode: 0x1f, opname: "lload_1", oplen: 1 };
    pub const LLOAD_2: Opcode = Opcode { opcode: 0x20, opname: "lload_2", oplen: 1 };
    pub const LLOAD_3: Opcode = Opcode { opcode: 0x21, opname: "lload_3", oplen: 1 };
    pub const LMUL: Opcode = Opcode { opcode: 0x69, opname: "lmul", oplen: 1 };
    pub const LNEG: Opcode = Opcode { opcode: 0x75, opname: "lneg", oplen: 1 };
    pub const LOOKUPSWITCH: Opcode = Opcode { opcode: 0xab, opname: "lookupswitch", oplen: 11 };
    pub const LOR: Opcode = Opcode { opcode: 0x81, opname: "lor", oplen: 1 };
    pub const LREM: Opcode = Opcode { opcode: 0x71, opname: "lrem", oplen: 1 };
    pub const LRETURN: Opcode = Opcode { opcode: 0xad, opname: "lreturn", oplen: 1 };
    pub const LSHL: Opcode = Opcode { opcode: 0x79, opname: "lshl", oplen: 1 };
    pub const LSHR: Opcode = Opcode { opcode: 0x7b, opname: "lshr", oplen: 1 };
    pub const LSTORE: Opcode = Opcode { opcode: 0x37, opname: "lstore", oplen: 2 };
    pub const LSTORE_0: Opcode = Opcode { opcode: 0x3f, opname: "lstore_0", oplen: 1 };
    pub const LSTORE_1: Opcode = Opcode { opcode: 0x40, opname: "lstore_1", oplen: 1 };
    pub const LSTORE_2: Opcode = Opcode { opcode: 0x41, opname: "lstore_2", oplen: 1 };
    pub const LSTORE_3: Opcode = Opcode { opcode: 0x42, opname: "lstore_3", oplen: 1 };
    pub const LSUB: Opcode = Opcode { opcode: 0x65, opname: "lsub", oplen: 1 };
    pub const LUSHR: Opcode = Opcode { opcode: 0x7d, opname: "lushr", oplen: 1 };
    pub const LXOR: Opcode = Opcode { opcode: 0x83, opname: "lxor", oplen: 1 };
    pub const MONITORENTER: Opcode = Opcode { opcode: 0xc2, opname: "monitorenter", oplen: 1 };
    pub const MONITOREXIT: Opcode = Opcode { opcode: 0xc3, opname: "monitorexit", oplen: 1 };
    pub const MULTIANEWARRAY: Opcode = Opcode { opcode: 0xc5, opname: "multianewarray", oplen: 4 };
    pub const NEW: Opcode = Opcode { opcode: 0xbb, opname: "new", oplen: 3 };
    pub const NEWARRAY: Opcode = Opcode { opcode: 0xbc, opname: "newarray", oplen: 2 };
    pub const NOP: Opcode = Opcode { opcode: 0x00, opname: "nop", oplen: 1 };
    pub const POP: Opcode = Opcode { opcode: 0x57, opname: "pop", oplen: 1 };
    pub const POP2: Opcode = Opcode { opcode: 0x58, opname: "pop2", oplen: 1 };
    pub const PUTFIELD: Opcode = Opcode { opcode: 0xb5, opname: "putfield", oplen: 3 };
    pub const PUTSTATIC: Opcode = Opcode { opcode: 0xb3, opname: "putstatic", oplen: 3 };
    pub const RET: Opcode = Opcode { opcode: 0xa9, opname: "ret", oplen: 2 };
    pub const RETURN: Opcode = Opcode { opcode: 0xb1, opname: "return", oplen: 1 };
    pub const SALOAD: Opcode = Opcode { opcode: 0x35, opname: "saload", oplen: 1 };
    pub const SASTORE: Opcode = Opcode { opcode: 0x56, opname: "sastore", oplen: 1 };
    pub const SIPUSH: Opcode = Opcode { opcode: 0x11, opname: "sipush", oplen: 3 };
    pub const SWAP: Opcode = Opcode { opcode: 0x5f, opname: "swap", oplen: 1 };
    pub const TABLESWITCH: Opcode = Opcode { opcode: 0xaa, opname: "tableswitch", oplen: 15 };

}
pub const OPCODES_MAP: [Option<Opcode>; 255] = [
    Some(Opcodes::NOP),
    Some(Opcodes::ACONST_NULL),
    Some(Opcodes::ICONST_M1),
    Some(Opcodes::ICONST_0),
    Some(Opcodes::ICONST_1),
    Some(Opcodes::ICONST_2),
    Some(Opcodes::ICONST_3),
    Some(Opcodes::ICONST_4),
    Some(Opcodes::ICONST_5),
    Some(Opcodes::LCONST_0),
    Some(Opcodes::LCONST_1),
    Some(Opcodes::FCONST_0),
    Some(Opcodes::FCONST_1),
    Some(Opcodes::FCONST_2),
    Some(Opcodes::DCONST_0),
    Some(Opcodes::DCONST_1),
    Some(Opcodes::BIPUSH),
    Some(Opcodes::SIPUSH),
    Some(Opcodes::LDC),
    Some(Opcodes::LDC_W),
    Some(Opcodes::LDC2_W),
    Some(Opcodes::ILOAD),
    Some(Opcodes::LLOAD),
    Some(Opcodes::FLOAD),
    Some(Opcodes::DLOAD),
    Some(Opcodes::ALOAD),
    Some(Opcodes::ILOAD_0),
    Some(Opcodes::ILOAD_1),
    Some(Opcodes::ILOAD_2),
    Some(Opcodes::ILOAD_3),
    Some(Opcodes::LLOAD_0),
    Some(Opcodes::LLOAD_1),
    Some(Opcodes::LLOAD_2),
    Some(Opcodes::LLOAD_3),
    Some(Opcodes::FLOAD_0),
    Some(Opcodes::FLOAD_1),
    Some(Opcodes::FLOAD_2),
    Some(Opcodes::FLOAD_3),
    Some(Opcodes::DLOAD_0),
    Some(Opcodes::DLOAD_1),
    Some(Opcodes::DLOAD_2),
    Some(Opcodes::DLOAD_3),
    Some(Opcodes::ALOAD_0),
    Some(Opcodes::ALOAD_1),
    Some(Opcodes::ALOAD_2),
    Some(Opcodes::ALOAD_3),
    Some(Opcodes::IALOAD),
    Some(Opcodes::LALOAD),
    Some(Opcodes::FALOAD),
    Some(Opcodes::DALOAD),
    Some(Opcodes::AALOAD),
    Some(Opcodes::BALOAD),
    Some(Opcodes::CALOAD),
    Some(Opcodes::SALOAD),
    Some(Opcodes::ISTORE),
    Some(Opcodes::LSTORE),
    Some(Opcodes::FSTORE),
    Some(Opcodes::DSTORE),
    Some(Opcodes::ASTORE),
    Some(Opcodes::ISTORE_0),
    Some(Opcodes::ISTORE_1),
    Some(Opcodes::ISTORE_2),
    Some(Opcodes::ISTORE_3),
    Some(Opcodes::LSTORE_0),
    Some(Opcodes::LSTORE_1),
    Some(Opcodes::LSTORE_2),
    Some(Opcodes::LSTORE_3),
    Some(Opcodes::FSTORE_0),
    Some(Opcodes::FSTORE_1),
    Some(Opcodes::FSTORE_2),
    Some(Opcodes::FSTORE_3),
    Some(Opcodes::DSTORE_0),
    Some(Opcodes::DSTORE_1),
    Some(Opcodes::DSTORE_2),
    Some(Opcodes::DSTORE_3),
    Some(Opcodes::ASTORE_0),
    Some(Opcodes::ASTORE_1),
    Some(Opcodes::ASTORE_2),
    Some(Opcodes::ASTORE_3),
    Some(Opcodes::IASTORE),
    Some(Opcodes::LASTORE),
    Some(Opcodes::FASTORE),
    Some(Opcodes::DASTORE),
    Some(Opcodes::AASTORE),
    Some(Opcodes::BASTORE),
    Some(Opcodes::CASTORE),
    Some(Opcodes::SASTORE),
    Some(Opcodes::POP),
    Some(Opcodes::POP2),
    Some(Opcodes::DUP),
    Some(Opcodes::DUP_X1),
    Some(Opcodes::DUP_X2),
    Some(Opcodes::DUP2),
    Some(Opcodes::DUP2_X1),
    Some(Opcodes::DUP2_X2),
    Some(Opcodes::SWAP),
    Some(Opcodes::IADD),
    Some(Opcodes::LADD),
    Some(Opcodes::FADD),
    Some(Opcodes::DADD),
    Some(Opcodes::ISUB),
    Some(Opcodes::LSUB),
    Some(Opcodes::FSUB),
    Some(Opcodes::DSUB),
    Some(Opcodes::IMUL),
    Some(Opcodes::LMUL),
    Some(Opcodes::FMUL),
    Some(Opcodes::DMUL),
    Some(Opcodes::IDIV),
    Some(Opcodes::LDIV),
    Some(Opcodes::FDIV),
    Some(Opcodes::DDIV),
    Some(Opcodes::IREM),
    Some(Opcodes::LREM),
    Some(Opcodes::FREM),
    Some(Opcodes::DREM),
    Some(Opcodes::INEG),
    Some(Opcodes::LNEG),
    Some(Opcodes::FNEG),
    Some(Opcodes::DNEG),
    Some(Opcodes::ISHL),
    Some(Opcodes::LSHL),
    Some(Opcodes::ISHR),
    Some(Opcodes::LSHR),
    Some(Opcodes::IUSHR),
    Some(Opcodes::LUSHR),
    Some(Opcodes::IAND),
    Some(Opcodes::LAND),
    Some(Opcodes::IOR),
    Some(Opcodes::LOR),
    Some(Opcodes::IXOR),
    Some(Opcodes::LXOR),
    Some(Opcodes::IINC),
    Some(Opcodes::I2L),
    Some(Opcodes::I2F),
    Some(Opcodes::I2D),
    Some(Opcodes::L2I),
    Some(Opcodes::L2F),
    Some(Opcodes::L2D),
    Some(Opcodes::F2I),
    Some(Opcodes::F2L),
    Some(Opcodes::F2D),
    Some(Opcodes::D2I),
    Some(Opcodes::D2L),
    Some(Opcodes::D2F),
    Some(Opcodes::I2B),
    Some(Opcodes::I2C),
    Some(Opcodes::I2S),
    Some(Opcodes::LCMP),
    Some(Opcodes::FCMPL),
    Some(Opcodes::FCMPG),
    Some(Opcodes::DCMPL),
    Some(Opcodes::DCMPG),
    Some(Opcodes::IFEQ),
    Some(Opcodes::IFNE),
    Some(Opcodes::IFLT),
    Some(Opcodes::IFGE),
    Some(Opcodes::IFGT),
    Some(Opcodes::IFLE),
    Some(Opcodes::IF_ICMPEQ),
    Some(Opcodes::IF_ICMPNE),
    Some(Opcodes::IF_ICMPLT),
    Some(Opcodes::IF_ICMPGE),
    Some(Opcodes::IF_ICMPGT),
    Some(Opcodes::IF_ICMPLE),
    Some(Opcodes::IF_ACMPEQ),
    Some(Opcodes::IF_ACMPNE),
    Some(Opcodes::GOTO),
    Some(Opcodes::JSR),
    Some(Opcodes::RET),
    Some(Opcodes::TABLESWITCH),
    Some(Opcodes::LOOKUPSWITCH),
    Some(Opcodes::IRETURN),
    Some(Opcodes::LRETURN),
    Some(Opcodes::FRETURN),
    Some(Opcodes::DRETURN),
    Some(Opcodes::ARETURN),
    Some(Opcodes::RETURN),
    Some(Opcodes::GETSTATIC),
    Some(Opcodes::PUTSTATIC),
    Some(Opcodes::GETFIELD),
    Some(Opcodes::PUTFIELD),
    Some(Opcodes::INVOKEVIRTUAL),
    Some(Opcodes::INVOKESPECIAL),
    Some(Opcodes::INVOKESTATIC),
    Some(Opcodes::INVOKEINTERFACE),
    Some(Opcodes::INVOKEDYNAMIC),
    Some(Opcodes::NEW),
    Some(Opcodes::NEWARRAY),
    Some(Opcodes::ANEWARRAY),
    Some(Opcodes::ARRAYLENGTH),
    Some(Opcodes::ATHROW),
    Some(Opcodes::CHECKCAST),
    Some(Opcodes::INSTANCEOF),
    Some(Opcodes::MONITORENTER),
    Some(Opcodes::MONITOREXIT),
    None,
    Some(Opcodes::MULTIANEWARRAY),
    Some(Opcodes::IFNULL),
    Some(Opcodes::IFNONNULL),
    Some(Opcodes::GOTO_W),
    Some(Opcodes::JSR_W),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];
