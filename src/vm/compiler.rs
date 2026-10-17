use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub const DSVM_VERSION: u16 = 2;
pub const STRING_MAX_SIZE: usize = 256;
pub const REPEAT_MAX_SIZE: usize = 256;
pub const MAX_COMBO: usize = 10;

pub const MEM_END_ADDR: u16 = 0xFFFF;
pub const USER_VAR_START_ADDRESS: u16 = 0xF000;
pub const USER_VAR_BYTE_WIDTH: u16 = 4;
pub const USER_VAR_END_ADDRESS_INCLUSIVE: u16 = 0xF3FF;
pub const MAX_UDV_COUNT: usize =
    ((USER_VAR_END_ADDRESS_INCLUSIVE - USER_VAR_START_ADDRESS + 1) / USER_VAR_BYTE_WIDTH) as usize;
pub const PGV_START_ADDRESS: u16 = 0xFC00;
pub const PGV_BYTE_WIDTH: u16 = 4;
pub const INTERNAL_VAR_START_ADDRESS: u16 = 0xFE00;
pub const INTERNAL_VAR_BYTE_WIDTH: u16 = 4;

pub const STACK_BASE_ADDR: usize = 0xEFFF;
pub const MIN_STACK_SIZE_BYTES: usize = 512;
pub const STACK_MOAT_BYTES: usize = 16;
pub const MAX_BIN_SIZE: usize = STACK_BASE_ADDR - MIN_STACK_SIZE_BYTES - STACK_MOAT_BYTES;
pub const DUMMY_VAR_NAME: &str = "_DSVM_DUMMY";

pub const VAR_BOUNDARY_FP_REL: u8 = 0x1e;
pub const VAR_BOUNDARY_UDGV: u8 = 0x1f;

#[derive(Debug, Clone)]
pub struct CompileResult {
    pub is_success: bool,
    pub error_comment: String,
    pub error_line_number_starting_from_1: usize,
    pub error_line_str: String,
    pub bin_array: Vec<u8>,
}

impl CompileResult {
    pub fn err(line_num: usize, line_str: &str, msg: impl Into<String>) -> Self {
        Self {
            is_success: false,
            error_comment: msg.into(),
            error_line_number_starting_from_1: line_num,
            error_line_str: line_str.to_string(),
            bin_array: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Nop,
    Pushc16,
    Pushi,
    Pushr,
    Popi,
    Popr,
    Brz,
    Jmp,
    Alloc,
    Call,
    Ret,
    Halt,
    Push0,
    Push1,
    Drop,
    Dup,
    Randint,
    Randuint,
    Pushc32,
    Pushc8,
    Vmver,
    Peek8,
    Peeku8,
    Peek16,
    Peeku16,
    Peek32,
    Poke8,
    Poke16,
    Poke32,
    Eq,
    Noteq,
    Lt,
    Lte,
    Gt,
    Gte,
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Pow,
    Lshift,
    Rshift,
    Bitor,
    Bitxor,
    Bitand,
    Logiand,
    Logior,
    Ult,
    Ulte,
    Ugt,
    Ugte,
    Udiv,
    Umod,
    Lsr,
    Bitinv,
    Loginot,
    Usub,
    Delay,
    Kdown,
    Kup,
    Mscl,
    Mmov,
    Swcf,
    Swcc,
    Swcr,
    Str,
    Strln,
    OledCusr,
    OledPrnt,
    OledUpde,
    OledClr,
    OledRest,
    OledLine,
    OledRect,
    OledCirc,
    Bclr,
    Skipp,
    Gotop,
    Sleep,
    Randchr,
    Puts,
    Hidtx,
    // Virtual opcodes
    Pushstr,
}

impl Op {
    pub fn code(self) -> u8 {
        match self {
            Op::Nop => 0,
            Op::Pushc16 => 1,
            Op::Pushi => 2,
            Op::Pushr => 3,
            Op::Popi => 4,
            Op::Popr => 5,
            Op::Brz => 6,
            Op::Jmp => 7,
            Op::Alloc => 8,
            Op::Call => 9,
            Op::Ret => 10,
            Op::Halt => 11,
            Op::Push0 => 12,
            Op::Push1 => 13,
            Op::Drop => 14,
            Op::Dup => 15,
            Op::Randint => 16,
            Op::Randuint => 17,
            Op::Pushc32 => 18,
            Op::Pushc8 => 19,
            Op::Peek8 => 24,
            Op::Peeku8 => 25,
            Op::Peek16 => 26,
            Op::Peeku16 => 27,
            Op::Peek32 => 28,
            Op::Poke8 => 29,
            Op::Poke16 => 30,
            Op::Poke32 => 31,
            Op::Eq => 32,
            Op::Noteq => 33,
            Op::Lt => 34,
            Op::Lte => 35,
            Op::Gt => 36,
            Op::Gte => 37,
            Op::Add => 38,
            Op::Sub => 39,
            Op::Mult => 40,
            Op::Div => 41,
            Op::Mod => 42,
            Op::Pow => 43,
            Op::Lshift => 44,
            Op::Rshift => 45,
            Op::Bitor => 46,
            Op::Bitxor => 47,
            Op::Bitand => 48,
            Op::Logiand => 49,
            Op::Logior => 50,
            Op::Ult => 51,
            Op::Ulte => 52,
            Op::Ugt => 53,
            Op::Ugte => 54,
            Op::Udiv => 55,
            Op::Umod => 56,
            Op::Lsr => 57,
            Op::Bitinv => 60,
            Op::Loginot => 61,
            Op::Usub => 62,
            Op::Delay => 64,
            Op::Kdown => 65,
            Op::Kup => 66,
            Op::Mscl => 67,
            Op::Mmov => 68,
            Op::Swcf => 69,
            Op::Swcc => 70,
            Op::Swcr => 71,
            Op::Str => 72,
            Op::Strln => 73,
            Op::OledCusr => 74,
            Op::OledPrnt => 75,
            Op::OledUpde => 76,
            Op::OledClr => 77,
            Op::OledRest => 78,
            Op::OledLine => 79,
            Op::OledRect => 80,
            Op::OledCirc => 81,
            Op::Bclr => 82,
            Op::Skipp => 83,
            Op::Gotop => 84,
            Op::Sleep => 85,
            Op::Randchr => 86,
            Op::Puts => 87,
            Op::Hidtx => 88,
            Op::Pushstr => 128,
            Op::Vmver => 255,
        }
    }

    pub fn length(self) -> usize {
        match self {
            Op::Pushc16
            | Op::Pushi
            | Op::Pushr
            | Op::Popi
            | Op::Popr
            | Op::Brz
            | Op::Jmp
            | Op::Alloc
            | Op::Call
            | Op::Ret
            | Op::Vmver
            | Op::Pushstr => 3,
            Op::Pushc32 => 5,
            Op::Pushc8 => 2,
            _ => 1,
        }
    }

    pub fn is_pushc(self) -> bool {
        matches!(self, Op::Pushc8 | Op::Pushc16 | Op::Pushc32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SymType {
    GlobalVar,
    FuncArg,
    FuncLocalVar,
    ReservedVar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Payload {
    None,
    Int(i64),
    Sym(String),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Op,
    pub payload: Payload,
    pub label: Option<String>,
    pub comment: String,
    pub addr: usize,
    pub parent_func: Option<String>,
    pub var_type: Option<SymType>,
}

impl Instruction {
    pub fn new(opcode: Op) -> Self {
        Self {
            opcode,
            payload: Payload::None,
            label: None,
            comment: String::new(),
            addr: 0,
            parent_func: None,
            var_type: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DsLine {
    pub content: String,
    pub orig_lnum_sf1: isize,
    pub indent_level: usize,
}

pub fn default_stdlib_lines() -> Vec<String> {
    let code = r#"
FUN WAITKEY(key_id)
    WHILE 1
        VAR pressed = _BLOCKING_READKEY
        IF pressed == key_id
            RETURN 1
        END_IF
    END_WHILE
END_FUN

FUN MEMSET(addr, value, length)
    VAR i = 0
    WHILE i < length
        POKE8(addr + i, value)
        i = i + 1
    END_WHILE
END_FUN

FUN ABS(n)
    IF _UNSIGNED_MATH == 1
        RETURN n
    END_IF
    
    IF n < 0
        RETURN n * -1
    END_IF
    
    RETURN n
END_FUN

FUN MIN(a, b)
    IF a < b
        RETURN a
    END_IF
    RETURN b
END_FUN

FUN MAX(a, b)
    IF a > b
        RETURN a
    END_IF
    RETURN b
END_FUN
"#;
    code.lines().map(|s| s.to_string()).collect()
}

pub fn reserved_variables_map() -> HashMap<&'static str, u16> {
    let mut m = HashMap::new();
    let internal = [
        "_DEFAULTDELAY",
        "_DEFAULTCHARDELAY",
        "_CHARJITTER",
        "_RANDOM_MIN",
        "_RANDOM_MAX",
        "_RANDOM_INT",
        "_TIME_MS",
        "_READKEY",
        "_LOOP_SIZE",
        "_KEYPRESS_COUNT",
        "_EPILOGUE_ACTIONS",
        "_TIME_S",
        "_ALLOW_ABORT",
        "_BLOCKING_READKEY",
        "_KBLED_BITFIELD",
        "_DONT_REPEAT",
        "_THIS_KEYID",
        "_DP_MODEL",
        "_RTC_IS_VALID",
        "_RTC_UTC_OFFSET",
        "_RTC_YEAR",
        "_RTC_MONTH",
        "_RTC_DAY",
        "_RTC_HOUR",
        "_RTC_MINUTE",
        "_RTC_SECOND",
        "_RTC_WDAY",
        "_RTC_YDAY",
        "_SW_BITFIELD",
    ];
    for (i, name) in internal.iter().enumerate() {
        m.insert(
            *name,
            INTERNAL_VAR_START_ADDRESS + (i as u16) * INTERNAL_VAR_BYTE_WIDTH,
        );
    }
    let pgvs = [
        "_GV0", "_GV1", "_GV2", "_GV3", "_GV4", "_GV5", "_GV6", "_GV7", "_GV8", "_GV9", "_GV10",
        "_GV11", "_GV12", "_GV13", "_GV14", "_GV15", "_GV16", "_GV17", "_GV18", "_GV19", "_GV20",
        "_GV21", "_GV22", "_GV23", "_GV24", "_GV25", "_GV26", "_GV27", "_GV28", "_GV29", "_GV30",
        "_GV31",
    ];
    for (i, name) in pgvs.iter().enumerate() {
        m.insert(*name, PGV_START_ADDRESS + (i as u16) * PGV_BYTE_WIDTH);
    }
    m.insert(DUMMY_VAR_NAME, MEM_END_ADDR);
    m
}

pub fn hid_keyname_lookup(name: &str) -> Option<(u8, u8)> {
    const KEY_TYPE_CHAR: u8 = 1;
    const KEY_TYPE_MODIFIER: u8 = 2;
    const KEY_TYPE_SPECIAL: u8 = 3;
    const KEY_TYPE_MEDIA: u8 = 4;
    const KEY_TYPE_MOUSE_BUTTON: u8 = 11;

    Some(match name {
        "LMOUSE" => (1, KEY_TYPE_MOUSE_BUTTON),
        "RMOUSE" => (2, KEY_TYPE_MOUSE_BUTTON),
        "MMOUSE" => (4, KEY_TYPE_MOUSE_BUTTON),
        "BMOUSE" => (8, KEY_TYPE_MOUSE_BUTTON),
        "FMOUSE" => (16, KEY_TYPE_MOUSE_BUTTON),

        "UP" | "UPARROW" => (0x52, KEY_TYPE_SPECIAL),
        "DOWN" | "DOWNARROW" => (0x51, KEY_TYPE_SPECIAL),
        "LEFT" | "LEFTARROW" => (0x50, KEY_TYPE_SPECIAL),
        "RIGHT" | "RIGHTARROW" => (0x4F, KEY_TYPE_SPECIAL),
        "ESCAPE" | "ESC" => (0x29, KEY_TYPE_SPECIAL),
        "ENTER" => (0x28, KEY_TYPE_SPECIAL),
        "BACKSPACE" => (0x2A, KEY_TYPE_SPECIAL),
        "TAB" => (0x2B, KEY_TYPE_SPECIAL),
        "CAPSLOCK" => (0x39, KEY_TYPE_SPECIAL),
        "PRINTSCREEN" => (0x46, KEY_TYPE_SPECIAL),
        "SCROLLLOCK" => (0x47, KEY_TYPE_SPECIAL),
        "PAUSE" | "BREAK" => (0x48, KEY_TYPE_SPECIAL),
        "INSERT" => (0x49, KEY_TYPE_SPECIAL),
        "HOME" => (0x4A, KEY_TYPE_SPECIAL),
        "PAGEUP" => (0x4B, KEY_TYPE_SPECIAL),
        "PAGEDOWN" => (0x4E, KEY_TYPE_SPECIAL),
        "DELETE" => (0x4C, KEY_TYPE_SPECIAL),
        "END" => (0x4D, KEY_TYPE_SPECIAL),
        "NUMLOCK" => (0x53, KEY_TYPE_SPECIAL),
        "KP_SLASH" => (0x54, KEY_TYPE_SPECIAL),
        "KP_ASTERISK" => (0x55, KEY_TYPE_SPECIAL),
        "KP_MINUS" => (0x56, KEY_TYPE_SPECIAL),
        "KP_PLUS" => (0x57, KEY_TYPE_SPECIAL),
        "KP_ENTER" => (0x58, KEY_TYPE_SPECIAL),
        "KP_1" => (0x59, KEY_TYPE_SPECIAL),
        "KP_2" => (0x5A, KEY_TYPE_SPECIAL),
        "KP_3" => (0x5B, KEY_TYPE_SPECIAL),
        "KP_4" => (0x5C, KEY_TYPE_SPECIAL),
        "KP_5" => (0x5D, KEY_TYPE_SPECIAL),
        "KP_6" => (0x5E, KEY_TYPE_SPECIAL),
        "KP_7" => (0x5F, KEY_TYPE_SPECIAL),
        "KP_8" => (0x60, KEY_TYPE_SPECIAL),
        "KP_9" => (0x61, KEY_TYPE_SPECIAL),
        "KP_0" => (0x62, KEY_TYPE_SPECIAL),
        "KP_DOT" => (0x63, KEY_TYPE_SPECIAL),
        "KP_EQUAL" => (0x67, KEY_TYPE_SPECIAL),
        "POWER" => (0x66, KEY_TYPE_SPECIAL),
        "MENU" | "APP" => (0x65, KEY_TYPE_SPECIAL),

        "RO" => (0x87, KEY_TYPE_SPECIAL),
        "KATAKANAHIRAGANA" => (0x88, KEY_TYPE_SPECIAL),
        "YEN" => (0x89, KEY_TYPE_SPECIAL),
        "HENKAN" => (0x8A, KEY_TYPE_SPECIAL),
        "MUHENKAN" => (0x8B, KEY_TYPE_SPECIAL),
        "KPJPCOMMA" => (0x8C, KEY_TYPE_SPECIAL),
        "HANGEUL" => (0x90, KEY_TYPE_SPECIAL),
        "HANJA" => (0x91, KEY_TYPE_SPECIAL),
        "KATAKANA" => (0x92, KEY_TYPE_SPECIAL),
        "HIRAGANA" => (0x93, KEY_TYPE_SPECIAL),
        "ZENKAKUHANKAKU" => (0x94, KEY_TYPE_SPECIAL),

        "F1" => (0x3A, KEY_TYPE_SPECIAL),
        "F2" => (0x3B, KEY_TYPE_SPECIAL),
        "F3" => (0x3C, KEY_TYPE_SPECIAL),
        "F4" => (0x3D, KEY_TYPE_SPECIAL),
        "F5" => (0x3E, KEY_TYPE_SPECIAL),
        "F6" => (0x3F, KEY_TYPE_SPECIAL),
        "F7" => (0x40, KEY_TYPE_SPECIAL),
        "F8" => (0x41, KEY_TYPE_SPECIAL),
        "F9" => (0x42, KEY_TYPE_SPECIAL),
        "F10" => (0x43, KEY_TYPE_SPECIAL),
        "F11" => (0x44, KEY_TYPE_SPECIAL),
        "F12" => (0x45, KEY_TYPE_SPECIAL),
        "F13" => (0x68, KEY_TYPE_SPECIAL),
        "F14" => (0x69, KEY_TYPE_SPECIAL),
        "F15" => (0x6A, KEY_TYPE_SPECIAL),
        "F16" => (0x6B, KEY_TYPE_SPECIAL),
        "F17" => (0x6C, KEY_TYPE_SPECIAL),
        "F18" => (0x6D, KEY_TYPE_SPECIAL),
        "F19" => (0x6E, KEY_TYPE_SPECIAL),
        "F20" => (0x6F, KEY_TYPE_SPECIAL),
        "F21" => (0x70, KEY_TYPE_SPECIAL),
        "F22" => (0x71, KEY_TYPE_SPECIAL),
        "F23" => (0x72, KEY_TYPE_SPECIAL),
        "F24" => (0x73, KEY_TYPE_SPECIAL),

        "SPACE" => (0x20, KEY_TYPE_CHAR),
        "SHIFT" => (0x02, KEY_TYPE_MODIFIER),
        "RSHIFT" => (0x20, KEY_TYPE_MODIFIER),
        "ALT" | "OPTION" => (0x04, KEY_TYPE_MODIFIER),
        "RALT" | "ROPTION" => (0x40, KEY_TYPE_MODIFIER),
        "GUI" | "WINDOWS" | "COMMAND" => (0x08, KEY_TYPE_MODIFIER),
        "RWINDOWS" | "RCOMMAND" => (0x80, KEY_TYPE_MODIFIER),
        "CTRL" | "CONTROL" => (0x01, KEY_TYPE_MODIFIER),
        "RCTRL" | "RCONTROL" => (0x10, KEY_TYPE_MODIFIER),

        "MK_VOLUP" => (0x40, KEY_TYPE_MEDIA),
        "MK_VOLDOWN" => (0x80, KEY_TYPE_MEDIA),
        "MK_MUTE" => (0x20, KEY_TYPE_MEDIA),
        "MK_EJECT" => (0x08, KEY_TYPE_MEDIA),
        "MK_PREV" => (0x02, KEY_TYPE_MEDIA),
        "MK_NEXT" => (0x01, KEY_TYPE_MEDIA),
        "MK_PP" => (0x10, KEY_TYPE_MEDIA),
        "MK_STOP" => (0x04, KEY_TYPE_MEDIA),
        _ => return None,
    })
}

pub fn get_key_combined_value(keyname: &str) -> Result<u16, String> {
    if let Some((code, ktype)) = hid_keyname_lookup(keyname) {
        return Ok(((ktype as u16) << 8) | (code as u16));
    }
    if keyname.len() == 1 {
        let code = keyname.as_bytes()[0];
        return Ok((1u16 << 8) | (code as u16));
    }
    Err(format!("Invalid Key: {keyname}"))
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltinFuncInfo {
    pub opcode: Op,
    pub arg_len: usize,
    pub has_return_value: bool,
    pub is_str_func: bool,
    pub is_keypress_func: bool,
}

pub fn builtin_func_lookup(name: &str) -> Option<BuiltinFuncInfo> {
    Some(match name {
        // String functions
        "STRING" => BuiltinFuncInfo {
            opcode: Op::Str,
            arg_len: 1,
            has_return_value: false,
            is_str_func: true,
            is_keypress_func: false,
        },
        "STRINGLN" => BuiltinFuncInfo {
            opcode: Op::Strln,
            arg_len: 1,
            has_return_value: false,
            is_str_func: true,
            is_keypress_func: false,
        },
        "OLED_OPTPRINT" => BuiltinFuncInfo {
            opcode: Op::OledPrnt,
            arg_len: 2,
            has_return_value: false,
            is_str_func: true,
            is_keypress_func: false,
        },
        "GOTO_PROFILE" => BuiltinFuncInfo {
            opcode: Op::Gotop,
            arg_len: 1,
            has_return_value: false,
            is_str_func: true,
            is_keypress_func: false,
        },
        // Keypress functions
        "KEYDOWN" => BuiltinFuncInfo {
            opcode: Op::Kdown,
            arg_len: 1,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: true,
        },
        "KEYUP" => BuiltinFuncInfo {
            opcode: Op::Kup,
            arg_len: 1,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: true,
        },
        // Numeric / Void Built-in Functions
        "HALT" => BuiltinFuncInfo {
            opcode: Op::Halt,
            arg_len: 0,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "DELAY" => BuiltinFuncInfo {
            opcode: Op::Delay,
            arg_len: 1,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "MOUSE_SCROLL" => BuiltinFuncInfo {
            opcode: Op::Mscl,
            arg_len: 2,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "MOUSE_MOVE" => BuiltinFuncInfo {
            opcode: Op::Mmov,
            arg_len: 2,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "SWC_FILL" => BuiltinFuncInfo {
            opcode: Op::Swcf,
            arg_len: 3,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "SWC_SET" => BuiltinFuncInfo {
            opcode: Op::Swcc,
            arg_len: 4,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "SWC_RESET" => BuiltinFuncInfo {
            opcode: Op::Swcr,
            arg_len: 1,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "OLED_CURSOR" => BuiltinFuncInfo {
            opcode: Op::OledCusr,
            arg_len: 2,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "OLED_UPDATE" => BuiltinFuncInfo {
            opcode: Op::OledUpde,
            arg_len: 0,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "OLED_CLEAR" => BuiltinFuncInfo {
            opcode: Op::OledClr,
            arg_len: 0,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "OLED_RESTORE" => BuiltinFuncInfo {
            opcode: Op::OledRest,
            arg_len: 0,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "OLED_LINE" => BuiltinFuncInfo {
            opcode: Op::OledLine,
            arg_len: 4,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "OLED_RECT" => BuiltinFuncInfo {
            opcode: Op::OledRect,
            arg_len: 5,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "OLED_CIRCLE" => BuiltinFuncInfo {
            opcode: Op::OledCirc,
            arg_len: 4,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "BCLR" => BuiltinFuncInfo {
            opcode: Op::Bclr,
            arg_len: 0,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "SKIP_PROFILE" => BuiltinFuncInfo {
            opcode: Op::Skipp,
            arg_len: 1,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "DP_SLEEP" => BuiltinFuncInfo {
            opcode: Op::Sleep,
            arg_len: 0,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "RANDCHR" => BuiltinFuncInfo {
            opcode: Op::Randchr,
            arg_len: 1,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "RANDINT" => BuiltinFuncInfo {
            opcode: Op::Randint,
            arg_len: 2,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "RANDUINT" => BuiltinFuncInfo {
            opcode: Op::Randuint,
            arg_len: 2,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "PUTS" => BuiltinFuncInfo {
            opcode: Op::Puts,
            arg_len: 1,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "HIDTX" => BuiltinFuncInfo {
            opcode: Op::Hidtx,
            arg_len: 1,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "PEEK8" => BuiltinFuncInfo {
            opcode: Op::Peek8,
            arg_len: 1,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "PEEKU8" => BuiltinFuncInfo {
            opcode: Op::Peeku8,
            arg_len: 1,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "PEEK16" => BuiltinFuncInfo {
            opcode: Op::Peek16,
            arg_len: 1,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "PEEKU16" => BuiltinFuncInfo {
            opcode: Op::Peeku16,
            arg_len: 1,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "PEEK32" => BuiltinFuncInfo {
            opcode: Op::Peek32,
            arg_len: 1,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "POKE8" => BuiltinFuncInfo {
            opcode: Op::Poke8,
            arg_len: 2,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "POKE16" => BuiltinFuncInfo {
            opcode: Op::Poke16,
            arg_len: 2,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "POKE32" => BuiltinFuncInfo {
            opcode: Op::Poke32,
            arg_len: 2,
            has_return_value: false,
            is_str_func: false,
            is_keypress_func: false,
        },
        "ULT" => BuiltinFuncInfo {
            opcode: Op::Ult,
            arg_len: 2,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "ULTE" => BuiltinFuncInfo {
            opcode: Op::Ulte,
            arg_len: 2,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "UGT" => BuiltinFuncInfo {
            opcode: Op::Ugt,
            arg_len: 2,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "UGTE" => BuiltinFuncInfo {
            opcode: Op::Ugte,
            arg_len: 2,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "UDIV" => BuiltinFuncInfo {
            opcode: Op::Udiv,
            arg_len: 2,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "UMOD" => BuiltinFuncInfo {
            opcode: Op::Umod,
            arg_len: 2,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        "LSR" => BuiltinFuncInfo {
            opcode: Op::Lsr,
            arg_len: 2,
            has_return_value: true,
            is_str_func: false,
            is_keypress_func: false,
        },
        _ => return None,
    })
}

fn is_str_or_keypress_cmd(first_word: &str) -> bool {
    matches!(
        first_word,
        "STRING"
            | "STRINGLN"
            | "OLED_OPTPRINT"
            | "GOTO_PROFILE"
            | "OLED_PRINT"
            | "OLED_LPRINT"
            | "OLED_CPRINT"
            | "KEYDOWN"
            | "KEYUP"
    )
}

fn needs_rstrip(first_word: &str) -> bool {
    !matches!(first_word, "STRING" | "STRINGLN" | "OLED_PRINT")
}

fn default_defines() -> HashMap<String, String> {
    let mut d = HashMap::new();
    d.insert("RANDOM_LOWERCASE_LETTER".into(), "RANDCHR(0x101)".into());
    d.insert("RANDOM_UPPERCASE_LETTER".into(), "RANDCHR(0x102)".into());
    d.insert("RANDOM_LETTER".into(), "RANDCHR(0x103)".into());
    d.insert("RANDOM_NUMBER".into(), "RANDCHR(0x104)".into());
    d.insert("RANDOM_SPECIAL".into(), "RANDCHR(0x108)".into());
    d.insert("RANDOM_CHAR".into(), "RANDCHR(0x10f)".into());
    d.insert("DEFAULTDELAY".into(), "_DEFAULTDELAY=".into());
    d.insert("DEFAULTCHARDELAY".into(), "_DEFAULTCHARDELAY=".into());
    d.insert("CHARJITTER".into(), "_CHARJITTER=".into());
    d.insert("MOUSE_WHEEL".into(), "MOUSE_SCROLL 0".into());
    d.insert("FUNCTION ".into(), "FUN ".into());
    d.insert("END_FUNCTION".into(), "END_FUN".into());
    d.insert("TRUE".into(), "1".into());
    d.insert("FALSE".into(), "0".into());
    d.insert("OLED_PRINT".into(), "OLED_LPRINT".into());
    d.insert("_IS_NUMLOCK_ON".into(), "((_KBLED_BITFIELD&0x1)!=0)".into());
    d.insert(
        "_IS_CAPSLOCK_ON".into(),
        "((_KBLED_BITFIELD&0x2)!=0)".into(),
    );
    d.insert(
        "_IS_SCROLLLOCK_ON".into(),
        "((_KBLED_BITFIELD&0x4)!=0)".into(),
    );
    d.insert("PASS".into(), "pass".into());
    d.insert("_STR_PRINT_FORMAT".into(), DUMMY_VAR_NAME.into());
    d.insert("_STR_PRINT_PADDING".into(), DUMMY_VAR_NAME.into());
    d.insert("SWCOLOR".into(), "SWC_SET 0".into());
    d
}

fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn replace_word_boundary(haystack: &str, needle: &str, replacement: &str) -> (String, usize) {
    if needle.is_empty() || haystack.len() < needle.len() {
        return (haystack.to_string(), 0);
    }
    let h_bytes = haystack.as_bytes();
    let n_bytes = needle.as_bytes();
    let start_is_word = is_word_char(n_bytes[0]);
    let end_is_word = is_word_char(n_bytes[n_bytes.len() - 1]);

    let mut out = String::with_capacity(haystack.len());
    let mut i = 0;
    let mut count = 0;

    while i <= haystack.len() - needle.len() {
        if haystack[i..].starts_with(needle) {
            let left_ok = if start_is_word {
                i == 0 || !is_word_char(h_bytes[i - 1])
            } else {
                i == 0 || !is_word_char(h_bytes[i - 1])
            };
            let right_idx = i + needle.len();
            let right_ok = if end_is_word {
                right_idx == haystack.len() || !is_word_char(h_bytes[right_idx])
            } else {
                right_idx == haystack.len() || is_word_char(h_bytes[right_idx])
            };

            if left_ok && right_ok {
                out.push_str(replacement);
                count += 1;
                i += needle.len();
                continue;
            }
        }
        let ch = haystack[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out.push_str(&haystack[i..]);
    (out, count)
}

fn replace_define_once(pgm_line: &str, def_dict: &mut HashMap<String, String>) -> String {
    if pgm_line.starts_with("STRING ") || pgm_line.starts_with("STRINGLN ") {
        def_dict.remove("TRUE");
        def_dict.remove("FALSE");
    } else {
        def_dict.insert("TRUE".into(), "1".into());
        def_dict.insert("FALSE".into(), "0".into());
    }

    let mut keys: Vec<String> = def_dict.keys().cloned().collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));

    for key in keys {
        let val = &def_dict[&key];
        let (replaced, count) = replace_word_boundary(pgm_line, &key, val);
        if count > 0 {
            return replaced;
        }
    }
    pgm_line.to_string()
}

fn replace_define(source: &str, def_dict: &mut HashMap<String, String>) -> Result<String, String> {
    let mut current = source.to_string();
    let mut last = String::new();
    let mut iterations = 0;
    let max_iter = 512usize.max(def_dict.len() * 2);
    while last != current {
        if iterations > max_iter {
            return Err("Recursive DEFINE".to_string());
        }
        last = current.clone();
        current = replace_define_once(&current, def_dict);
        iterations += 1;
    }
    Ok(current)
}

pub fn replace_operators(this_line: &str) -> String {
    if this_line.trim_start().starts_with("DEFINE") {
        return this_line.to_string();
    }
    let placeholder = "__DSVM_NEQ_PLACEHOLDER__";
    let temp = this_line.replace("!=", placeholder);
    let temp = temp
        .replace('$', "")
        .replace("||", " or ")
        .replace("&&", " and ")
        .replace('!', " not ");
    temp.replace(placeholder, "!=")
}

fn split_string_chunks(s: &str, max_len: usize) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_len {
        return vec![s.to_string()];
    }
    chars
        .chunks(max_len)
        .map(|c| c.iter().collect::<String>())
        .collect()
}

fn check_loop(pgm_line: &str) -> Result<usize, String> {
    let rest = pgm_line.strip_prefix("LOOP").ok_or("Invalid LOOP")?;
    let Some((num_part, after)) = rest.split_once(':') else {
        return Err("missing \":\"".to_string());
    };
    if !after.trim().is_empty() {
        return Err("wrong number of arguments".to_string());
    }
    num_part
        .trim()
        .parse::<usize>()
        .map_err(|_| "Invalid LOOP number".to_string())
}

struct PreprocPassResult {
    rem_blocks: Vec<(isize, isize)>,
    strlen_blocks: Vec<(isize, isize)>,
    str_blocks: Vec<(isize, isize)>,
    loop_state_save_needed: bool,
    loop_size: Option<usize>,
    user_declared_vars: HashMap<Option<String>, HashSet<String>>,
    lines: Vec<DsLine>,
}

fn single_pass(
    mut program_listing: Vec<DsLine>,
    define_dict: &mut HashMap<String, String>,
) -> Result<PreprocPassResult, CompileResult> {
    let mut loop_numbers = BTreeSet::new();
    let mut func_stack: Vec<(String, isize)> = Vec::new();
    let mut if_stack: Vec<(isize, bool)> = Vec::new();
    let mut while_stack: Vec<isize> = Vec::new();
    let mut rem_stack: Vec<isize> = Vec::new();
    let mut rem_blocks: Vec<(isize, Option<isize>)> = Vec::new();
    let mut strlen_stack: Vec<isize> = Vec::new();
    let mut strlen_blocks: Vec<(isize, Option<isize>)> = Vec::new();
    let mut str_stack: Vec<isize> = Vec::new();
    let mut str_blocks: Vec<(isize, Option<isize>)> = Vec::new();
    let mut user_declared_vars: HashMap<Option<String>, HashSet<String>> = HashMap::new();
    let reserved = reserved_variables_map();

    for line_obj in &mut program_listing {
        let lnum = line_obj.orig_lnum_sf1;
        let lnum_usize = lnum.unsigned_abs();
        let trimmed_start = line_obj.content.trim_start_matches([' ', '\t']);
        if trimmed_start.is_empty() {
            continue;
        }
        let first_word = trimmed_start.split_whitespace().next().unwrap_or("");
        let mut this_line = if needs_rstrip(first_word) {
            trimmed_start.trim_end_matches([' ', '\t']).to_string()
        } else {
            trimmed_start.to_string()
        };

        let mut indent_lvl =
            (if_stack.len() + func_stack.len() + while_stack.len()) as isize;

        let in_rem = rem_blocks
            .iter()
            .any(|&(s, e)| e.map_or(lnum >= s, |end| lnum >= s && lnum <= end));
        let in_strlen = strlen_blocks
            .iter()
            .any(|&(s, e)| e.map_or(true, |end| lnum > s && lnum < end));
        let in_str = str_blocks
            .iter()
            .any(|&(s, e)| e.map_or(true, |end| lnum > s && lnum < end));

        if first_word != "DEFINE" && !in_rem && !in_strlen && !in_str {
            match replace_define(&this_line, define_dict) {
                Ok(rep) => this_line = rep,
                Err(e) => return Err(CompileResult::err(lnum_usize, &this_line, e)),
            }
        }
        let first_word = this_line.split_whitespace().next().unwrap_or("");

        if first_word == "END_REM" {
            let Some(start) = rem_stack.pop() else {
                return Err(CompileResult::err(lnum_usize, &this_line, "orphan END_REM"));
            };
            if let Some(entry) = rem_blocks.iter_mut().find(|x| x.0 == start) {
                entry.1 = Some(lnum);
            }
        } else if in_rem {
            // inside REM_BLOCK
        } else if first_word == "END_STRINGLN" {
            let Some(start) = strlen_stack.pop() else {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "orphan END_STRINGLN",
                ));
            };
            if let Some(entry) = strlen_blocks.iter_mut().find(|x| x.0 == start) {
                entry.1 = Some(lnum);
            }
        } else if in_strlen {
            // inside STRINGLN_BLOCK
        } else if first_word == "END_STRING" {
            let Some(start) = str_stack.pop() else {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "orphan END_STRING",
                ));
            };
            if let Some(entry) = str_blocks.iter_mut().find(|x| x.0 == start) {
                entry.1 = Some(lnum);
            }
        } else if in_str {
            // inside STRING_BLOCK
        } else if first_word == "REM_BLOCK" {
            if !rem_stack.is_empty() {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "unmatched END_REM",
                ));
            }
            rem_stack.push(lnum);
            rem_blocks.push((lnum, None));
        } else if first_word == "STRINGLN_BLOCK" {
            if !strlen_stack.is_empty() {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "unmatched END_STRINGLN",
                ));
            }
            strlen_stack.push(lnum);
            strlen_blocks.push((lnum, None));
        } else if first_word == "STRING_BLOCK" {
            if !str_stack.is_empty() {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "unmatched END_STRING",
                ));
            }
            str_stack.push(lnum);
            str_blocks.push((lnum, None));
        } else if first_word == "DEFINE" {
            let clean = this_line
                .rsplit_once("//")
                .map(|(l, _)| l)
                .unwrap_or(&this_line)
                .trim_end();
            let rest = clean.strip_prefix("DEFINE").unwrap_or("").trim_start();
            let Some((def_key, def_val)) = rest.split_once(' ') else {
                return Err(CompileResult::err(lnum_usize, &this_line, "empty DEFINE"));
            };
            define_dict.insert(def_key.trim().to_string(), def_val.to_string());
        } else if first_word == "VAR" {
            let op_line = replace_operators(&this_line);
            let after_var = op_line.strip_prefix("VAR").unwrap_or("");
            let Some((lhs, rhs)) = after_var.split_once('=') else {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "Invalid VAR declaration",
                ));
            };
            let var_name = lhs.trim();
            if rhs.trim().is_empty() || var_name.is_empty() {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "Empty VAR declaration",
                ));
            }
            if reserved.contains_key(var_name) {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "Re-declaration of reserved variable",
                ));
            }
            let parent_fn = func_stack.last().map(|(n, _)| n.clone());
            let set = user_declared_vars.entry(parent_fn).or_default();
            if !set.insert(var_name.to_string()) {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "Duplicate var name",
                ));
            }
        } else if first_word == "FUN" {
            if !func_stack.is_empty() {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "unmatched END_FUN",
                ));
            }
            if !this_line.ends_with(')') {
                return Err(CompileResult::err(lnum_usize, &this_line, "missing )"));
            }
            let after_fun = this_line.strip_prefix("FUN").unwrap_or("").trim();
            let Some((fn_name, _)) = after_fun.split_once('(') else {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "invalid func name",
                ));
            };
            func_stack.push((fn_name.trim().to_string(), lnum));
        } else if first_word == "END_FUN" {
            indent_lvl -= 1;
            if func_stack.pop().is_none() {
                return Err(CompileResult::err(lnum_usize, &this_line, "orphan END_FUN"));
            }
        } else if first_word == "IF" {
            if_stack.push((lnum, false));
        } else if this_line.starts_with("ELSE IF") {
            indent_lvl -= 1;
            let Some(top) = if_stack.last() else {
                return Err(CompileResult::err(lnum_usize, &this_line, "orphan ELSE IF"));
            };
            if top.1 {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "ELSE IF must be before ELSE",
                ));
            }
        } else if first_word == "ELSE" {
            indent_lvl -= 1;
            let Some(top) = if_stack.last_mut() else {
                return Err(CompileResult::err(lnum_usize, &this_line, "orphan ELSE"));
            };
            if top.1 {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "unmatched ELSE",
                ));
            }
            top.1 = true;
        } else if first_word == "END_IF" {
            indent_lvl -= 1;
            if if_stack.pop().is_none() {
                return Err(CompileResult::err(lnum_usize, &this_line, "orphan END_IF"));
            }
        } else if first_word == "WHILE" {
            while_stack.push(lnum);
        } else if first_word == "END_WHILE" {
            indent_lvl -= 1;
            if while_stack.pop().is_none() {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "orphan END_WHILE",
                ));
            }
        } else if first_word == "LBREAK" {
            if while_stack.is_empty() {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "BREAK outside WHILE",
                ));
            }
        } else if first_word == "CONTINUE" {
            if while_stack.is_empty() {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "CONTINUE outside WHILE",
                ));
            }
        } else if first_word == "RETURN" {
            if func_stack.is_empty() {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "RETURN outside function",
                ));
            }
        } else if this_line.starts_with("LOOP") {
            match check_loop(&this_line) {
                Ok(v) => {
                    loop_numbers.insert(v);
                }
                Err(e) => return Err(CompileResult::err(lnum_usize, &this_line, e)),
            }
        }

        if indent_lvl < 0 {
            return Err(CompileResult::err(
                lnum_usize,
                &this_line,
                "Invalid indent level",
            ));
        }
        line_obj.indent_level = indent_lvl as usize;
    }

    if let Some((lnum, _)) = if_stack.first() {
        return Err(CompileResult::err(
            lnum.unsigned_abs(),
            "",
            "Missing END_IF",
        ));
    }
    if let Some((_, lnum)) = func_stack.first() {
        return Err(CompileResult::err(
            lnum.unsigned_abs(),
            "",
            "Missing END_FUN",
        ));
    }
    if let Some(lnum) = while_stack.last() {
        return Err(CompileResult::err(
            lnum.unsigned_abs(),
            "",
            "Missing END_WHILE",
        ));
    }
    if let Some(&(lnum, None)) = rem_blocks.iter().find(|x| x.1.is_none()) {
        return Err(CompileResult::err(
            lnum.unsigned_abs(),
            "",
            "Missing END_REM",
        ));
    }
    if let Some(&(lnum, None)) = strlen_blocks.iter().find(|x| x.1.is_none()) {
        return Err(CompileResult::err(
            lnum.unsigned_abs(),
            "",
            "Missing END_STRINGLN",
        ));
    }
    if let Some(&(lnum, None)) = str_blocks.iter().find(|x| x.1.is_none()) {
        return Err(CompileResult::err(
            lnum.unsigned_abs(),
            "",
            "Missing END_STRING",
        ));
    }

    Ok(PreprocPassResult {
        rem_blocks: rem_blocks
            .into_iter()
            .filter_map(|(s, e)| e.map(|end| (s, end)))
            .collect(),
        strlen_blocks: strlen_blocks
            .into_iter()
            .filter_map(|(s, e)| e.map(|end| (s, end)))
            .collect(),
        str_blocks: str_blocks
            .into_iter()
            .filter_map(|(s, e)| e.map(|end| (s, end)))
            .collect(),
        loop_state_save_needed: !loop_numbers.is_empty(),
        loop_size: loop_numbers.iter().copied().max(),
        user_declared_vars,
        lines: program_listing,
    })
}

fn preprocess_all(
    program_listing: Vec<DsLine>,
    stdlib_lines: Option<Vec<DsLine>>,
) -> Result<PreprocPassResult, CompileResult> {
    let mut all_def_dict = default_defines();
    let pass1 = single_pass(program_listing, &mut all_def_dict)?;

    let mut stage1 = Vec::new();
    let mut imported_stdlib = false;

    for mut line_obj in pass1.lines {
        let lnum = line_obj.orig_lnum_sf1;
        let in_strlen = pass1
            .strlen_blocks
            .iter()
            .any(|&(s, e)| lnum > s && lnum < e);
        let in_str = pass1.str_blocks.iter().any(|&(s, e)| lnum > s && lnum < e);
        let in_rem = pass1
            .rem_blocks
            .iter()
            .any(|&(s, e)| lnum >= s && lnum <= e);

        if in_strlen {
            line_obj.content = format!("STRINGLN {}", line_obj.content);
        } else if in_str {
            line_obj.content = format!("STRING {}", line_obj.content);
        } else if in_rem {
            line_obj.content = format!("// {}", line_obj.content);
        } else {
            line_obj.content = line_obj.content.trim_start_matches([' ', '\t']).to_string();
        }

        if line_obj.content.is_empty() {
            continue;
        }

        let first_word = line_obj.content.split(' ').next().unwrap_or("");
        if first_word == "DPDSSTDLIB" {
            if !imported_stdlib {
                imported_stdlib = true;
                if let Some(ref stdlib) = stdlib_lines {
                    stage1.extend(stdlib.clone());
                }
            }
            continue;
        }

        if matches!(
            first_word,
            "STRINGLN_BLOCK" | "END_STRINGLN" | "STRING_BLOCK" | "END_STRING"
        ) {
            continue;
        }

        if first_word == "STRINGLN" || first_word == "STRING" {
            let prefix = format!("{first_word} ");
            let str_content = line_obj
                .content
                .strip_prefix(&prefix)
                .unwrap_or(&line_obj.content);
            if str_content.chars().count() <= STRING_MAX_SIZE {
                stage1.push(line_obj);
            } else {
                for chunk in split_string_chunks(str_content, STRING_MAX_SIZE) {
                    stage1.push(DsLine {
                        content: format!("STRING {chunk}"),
                        orig_lnum_sf1: line_obj.orig_lnum_sf1,
                        indent_level: 0,
                    });
                }
                if first_word == "STRINGLN" {
                    stage1.push(DsLine {
                        content: "ENTER".to_string(),
                        orig_lnum_sf1: line_obj.orig_lnum_sf1,
                        indent_level: 0,
                    });
                }
            }
        } else {
            stage1.push(line_obj);
        }
    }

    let mut stage2 = Vec::new();
    for mut line_obj in stage1 {
        line_obj.content = line_obj.content.trim_start_matches([' ', '\t']).to_string();
        let first_word = line_obj.content.split(' ').next().unwrap_or("").to_string();
        if first_word == "REM" || first_word.starts_with("//") {
            continue;
        }
        if first_word == "INJECT_MOD" {
            line_obj.content = line_obj.content.replacen("INJECT_MOD", "", 1);
        }
        if !is_str_or_keypress_cmd(&first_word) {
            if let Some(cpos) = line_obj.content.find("//") {
                line_obj.content = line_obj.content[..cpos]
                    .trim_end_matches([' ', '\t'])
                    .to_string();
            }
            line_obj.content = replace_operators(&line_obj.content);
        }
        if first_word == "PREV_PROFILE" {
            line_obj.content = "SKIP_PROFILE -1".to_string();
        }
        if first_word == "NEXT_PROFILE" {
            line_obj.content = "SKIP_PROFILE 1".to_string();
        }
        if line_obj.content.trim() == "GOTO_PROFILE" {
            return Err(CompileResult::err(
                line_obj.orig_lnum_sf1.unsigned_abs(),
                &line_obj.content,
                "Missing profile name",
            ));
        }
        line_obj.content = line_obj.content.trim_start_matches([' ', '\t']).to_string();
        stage2.push(line_obj);
    }

    let pass2 = single_pass(stage2, &mut all_def_dict)?;

    let mut second_pass_listing: Vec<DsLine> = Vec::new();
    let mut needs_end_if = false;

    if pass2.loop_state_save_needed {
        second_pass_listing.push(DsLine {
            content: "_EPILOGUE_ACTIONS = 1".to_string(),
            orig_lnum_sf1: 0,
            indent_level: 0,
        });
    }
    if let Some(ls) = pass2.loop_size {
        second_pass_listing.push(DsLine {
            content: format!("_LOOP_SIZE = {}", ls + 1),
            orig_lnum_sf1: 0,
            indent_level: 0,
        });
    }

    let mut last_lnum = 0isize;
    for mut line_obj in pass2.lines {
        let lnum = line_obj.orig_lnum_sf1;
        last_lnum = lnum;
        let lnum_usize = lnum.unsigned_abs();
        let this_line = line_obj.content.trim_start_matches([' ', '\t']).to_string();
        if this_line.is_empty() {
            continue;
        }
        let first_word = this_line.split(' ').next().unwrap_or("");
        if needs_rstrip(first_word) {
            line_obj.content = this_line.trim_end_matches([' ', '\t']).to_string();
        } else {
            line_obj.content = this_line.clone();
        }
        if first_word != "DEFINE" {
            line_obj.content = replace_define(&line_obj.content, &mut all_def_dict)
                .map_err(|e| CompileResult::err(lnum_usize, &this_line, e))?;
        } else {
            continue;
        }
        let this_line = line_obj.content.trim_start_matches([' ', '\t']).to_string();
        let first_word = this_line.split(' ').next().unwrap_or("");

        if first_word == "REPEAT" {
            let Some(prev) = second_pass_listing.last().cloned() else {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "Nothing before REPEAT",
                ));
            };
            let count_str = this_line.strip_prefix("REPEAT").unwrap_or("").trim();
            let Ok(repeat_count) = count_str.parse::<usize>() else {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "Invalid REPEAT count",
                ));
            };
            if repeat_count > REPEAT_MAX_SIZE {
                return Err(CompileResult::err(
                    lnum_usize,
                    &this_line,
                    "Invalid REPEAT count",
                ));
            }
            for _ in 0..repeat_count {
                second_pass_listing.push(prev.clone());
            }
        } else if this_line.starts_with("LOOP") {
            let value = check_loop(&this_line)
                .map_err(|e| CompileResult::err(lnum_usize, &this_line, e))?;
            if needs_end_if {
                second_pass_listing.push(DsLine {
                    content: "END_IF".to_string(),
                    orig_lnum_sf1: lnum,
                    indent_level: 0,
                });
            }
            second_pass_listing.push(DsLine {
                content: format!("IF _KEYPRESS_COUNT % _LOOP_SIZE == {value}"),
                orig_lnum_sf1: lnum,
                indent_level: 0,
            });
            needs_end_if = true;
        } else {
            second_pass_listing.push(line_obj);
        }
    }

    if needs_end_if {
        second_pass_listing.push(DsLine {
            content: "END_IF".to_string(),
            orig_lnum_sf1: last_lnum,
            indent_level: 0,
        });
    }

    // Expand key combos
    let mut expanded = Vec::new();
    for mut line_obj in second_pass_listing {
        let first_word = line_obj.content.split(' ').next().unwrap_or("");
        if hid_keyname_lookup(first_word).is_some() {
            let combo_keys: Vec<&str> = line_obj
                .content
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect();
            if combo_keys.len() > MAX_COMBO {
                return Err(CompileResult::err(
                    line_obj.orig_lnum_sf1.unsigned_abs(),
                    &line_obj.content,
                    format!("No more than {MAX_COMBO} combos"),
                ));
            }
            for &k in &combo_keys {
                if hid_keyname_lookup(k).is_none() && k.len() != 1 {
                    return Err(CompileResult::err(
                        line_obj.orig_lnum_sf1.unsigned_abs(),
                        &line_obj.content,
                        "Invalid combo character",
                    ));
                }
            }
            for &k in &combo_keys {
                expanded.push(DsLine {
                    content: format!("KEYDOWN {k}"),
                    orig_lnum_sf1: line_obj.orig_lnum_sf1,
                    indent_level: line_obj.indent_level,
                });
            }
            for &k in combo_keys.iter().rev() {
                expanded.push(DsLine {
                    content: format!("KEYUP {k}"),
                    orig_lnum_sf1: line_obj.orig_lnum_sf1,
                    indent_level: line_obj.indent_level,
                });
            }
        } else {
            line_obj.content = line_obj.content.trim_start_matches([' ', '\t']).to_string();
            expanded.push(line_obj);
        }
    }

    single_pass(expanded, &mut all_def_dict)
}

// ---------------- AST & Expression Parser ----------------

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Str(String),
    Var(String),
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    BoolOp {
        op: Op,
        values: Vec<Expr>,
    },
    UnaryOp {
        op: Op,
        operand: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign {
        target: String,
        value: Expr,
        orig_lnum: isize,
        orig_line: String,
    },
    AugAssign {
        target: String,
        op: Op,
        value: Expr,
        orig_lnum: isize,
        orig_line: String,
    },
    If {
        test: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
        orig_lnum: isize,
        orig_line: String,
    },
    While {
        test: Expr,
        body: Vec<Stmt>,
        orig_lnum: isize,
        orig_line: String,
    },
    FuncDef {
        name: String,
        args: Vec<String>,
        body: Vec<Stmt>,
        orig_lnum: isize,
        orig_line: String,
    },
    Return {
        value: Option<Expr>,
        orig_lnum: isize,
        orig_line: String,
    },
    Break {
        orig_lnum: isize,
        orig_line: String,
    },
    Continue {
        orig_lnum: isize,
        orig_line: String,
    },
    Pass {
        orig_lnum: isize,
        orig_line: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Int(i64),
    Str(String),
    Ident(String),
    LParen,
    RParen,
    Comma,
    Plus,
    Minus,
    Star,
    DoubleStar,
    Slash,
    DoubleSlash,
    Percent,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    BitNot,
    EqEq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    And,
    Or,
    Not,
}

fn tokenize_expr(input: &str) -> Result<Vec<Tok>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == '\'' || c == '"' {
            let quote = c;
            i += 1;
            let mut s = String::new();
            while i < chars.len() && chars[i] != quote {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1;
                    match chars[i] {
                        'n' => s.push('\n'),
                        'r' => s.push('\r'),
                        't' => s.push('\t'),
                        '\\' => s.push('\\'),
                        '\'' => s.push('\''),
                        '"' => s.push('"'),
                        other => s.push(other),
                    }
                } else {
                    s.push(chars[i]);
                }
                i += 1;
            }
            if i >= chars.len() {
                return Err("Unterminated string literal".to_string());
            }
            i += 1;
            tokens.push(Tok::Str(s));
            continue;
        }
        if c.is_ascii_digit() {
            let start = i;
            if c == '0' && i + 1 < chars.len() && (chars[i + 1] == 'x' || chars[i + 1] == 'X') {
                i += 2;
                let hex_start = i;
                while i < chars.len() && chars[i].is_ascii_hexdigit() {
                    i += 1;
                }
                let hex_str: String = chars[hex_start..i].iter().collect();
                let val = i64::from_str_radix(&hex_str, 16)
                    .map_err(|_| format!("Invalid hex literal: 0x{hex_str}"))?;
                tokens.push(Tok::Int(val));
            } else {
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let val = num_str
                    .parse::<i64>()
                    .map_err(|_| format!("Invalid integer: {num_str}"))?;
                tokens.push(Tok::Int(val));
            }
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let ident: String = chars[start..i].iter().collect();
            match ident.as_str() {
                "and" => tokens.push(Tok::And),
                "or" => tokens.push(Tok::Or),
                "not" => tokens.push(Tok::Not),
                _ => tokens.push(Tok::Ident(ident)),
            }
            continue;
        }
        // Two-character operators
        if i + 1 < chars.len() {
            let pair = (chars[i], chars[i + 1]);
            let two_tok = match pair {
                ('*', '*') => Some(Tok::DoubleStar),
                ('/', '/') => Some(Tok::DoubleSlash),
                ('<', '<') => Some(Tok::LShift),
                ('>', '>') => Some(Tok::RShift),
                ('=', '=') => Some(Tok::EqEq),
                ('!', '=') => Some(Tok::NotEq),
                ('<', '=') => Some(Tok::LtE),
                ('>', '=') => Some(Tok::GtE),
                _ => None,
            };
            if let Some(t) = two_tok {
                tokens.push(t);
                i += 2;
                continue;
            }
        }
        let single_tok = match c {
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            ',' => Tok::Comma,
            '+' => Tok::Plus,
            '-' => Tok::Minus,
            '*' => Tok::Star,
            '/' => Tok::Slash,
            '%' => Tok::Percent,
            '|' => Tok::BitOr,
            '^' => Tok::BitXor,
            '&' => Tok::BitAnd,
            '~' => Tok::BitNot,
            '<' => Tok::Lt,
            '>' => Tok::Gt,
            _ => return Err(format!("Unexpected character in expression: '{c}'")),
        };
        tokens.push(single_tok);
        i += 1;
    }

    Ok(tokens)
}

struct ExprParser {
    tokens: Vec<Tok>,
    pos: usize,
}

impl ExprParser {
    fn new(tokens: Vec<Tok>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Tok> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let first = self.parse_and()?;
        let mut values = vec![first];
        while matches!(self.peek(), Some(Tok::Or)) {
            self.next();
            values.push(self.parse_and()?);
        }
        if values.len() == 1 {
            Ok(values.pop().unwrap())
        } else {
            Ok(Expr::BoolOp {
                op: Op::Logior,
                values,
            })
        }
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let first = self.parse_not()?;
        let mut values = vec![first];
        while matches!(self.peek(), Some(Tok::And)) {
            self.next();
            values.push(self.parse_not()?);
        }
        if values.len() == 1 {
            Ok(values.pop().unwrap())
        } else {
            Ok(Expr::BoolOp {
                op: Op::Logiand,
                values,
            })
        }
    }

    fn parse_not(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), Some(Tok::Not)) {
            self.next();
            let operand = self.parse_not()?;
            return Ok(Expr::UnaryOp {
                op: Op::Loginot,
                operand: Box::new(operand),
            });
        }
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let left = self.parse_bitor()?;
        let cmp_op = match self.peek() {
            Some(Tok::EqEq) => Some(Op::Eq),
            Some(Tok::NotEq) => Some(Op::Noteq),
            Some(Tok::Lt) => Some(Op::Lt),
            Some(Tok::LtE) => Some(Op::Lte),
            Some(Tok::Gt) => Some(Op::Gt),
            Some(Tok::GtE) => Some(Op::Gte),
            _ => None,
        };
        if let Some(op) = cmp_op {
            self.next();
            let right = self.parse_bitor()?;
            Ok(Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_bitor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_bitxor()?;
        while matches!(self.peek(), Some(Tok::BitOr)) {
            self.next();
            let right = self.parse_bitxor()?;
            left = Expr::BinOp {
                op: Op::Bitor,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitxor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_bitand()?;
        while matches!(self.peek(), Some(Tok::BitXor)) {
            self.next();
            let right = self.parse_bitand()?;
            left = Expr::BinOp {
                op: Op::Bitxor,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitand(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_shift()?;
        while matches!(self.peek(), Some(Tok::BitAnd)) {
            self.next();
            let right = self.parse_shift()?;
            left = Expr::BinOp {
                op: Op::Bitand,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_add()?;
        loop {
            let op = match self.peek() {
                Some(Tok::LShift) => Op::Lshift,
                Some(Tok::RShift) => Op::Rshift,
                _ => break,
            };
            self.next();
            let right = self.parse_add()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_add(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul()?;
        loop {
            let op = match self.peek() {
                Some(Tok::Plus) => Op::Add,
                Some(Tok::Minus) => Op::Sub,
                _ => break,
            };
            self.next();
            let right = self.parse_mul()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_mul(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                Some(Tok::Star) => Op::Mult,
                Some(Tok::Slash) | Some(Tok::DoubleSlash) => Op::Div,
                Some(Tok::Percent) => Op::Mod,
                _ => break,
            };
            self.next();
            let right = self.parse_unary()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Tok::Minus) => {
                self.next();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: Op::Usub,
                    operand: Box::new(operand),
                })
            }
            Some(Tok::BitNot) => {
                self.next();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: Op::Bitinv,
                    operand: Box::new(operand),
                })
            }
            Some(Tok::Plus) => {
                self.next();
                self.parse_unary()
            }
            _ => self.parse_power(),
        }
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let left = self.parse_primary()?;
        if matches!(self.peek(), Some(Tok::DoubleStar)) {
            self.next();
            let right = self.parse_unary()?;
            Ok(Expr::BinOp {
                op: Op::Pow,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.next() {
            Some(Tok::Int(v)) => Ok(Expr::Int(v)),
            Some(Tok::Str(s)) => Ok(Expr::Str(s)),
            Some(Tok::Ident(name)) => {
                if matches!(self.peek(), Some(Tok::LParen)) {
                    self.next();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Tok::RParen)) {
                        loop {
                            args.push(self.parse_expr()?);
                            if matches!(self.peek(), Some(Tok::Comma)) {
                                self.next();
                                if matches!(self.peek(), Some(Tok::RParen)) {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    if !matches!(self.next(), Some(Tok::RParen)) {
                        return Err("Expected ')' after arguments".to_string());
                    }
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Some(Tok::LParen) => {
                let inner = self.parse_expr()?;
                if !matches!(self.next(), Some(Tok::RParen)) {
                    return Err("Expected ')'".to_string());
                }
                Ok(inner)
            }
            other => Err(format!("Unexpected token in expression: {other:?}")),
        }
    }
}

fn parse_expression_str(s: &str) -> Result<Expr, String> {
    let tokens = tokenize_expr(s)?;
    if tokens.is_empty() {
        return Err("Empty expression".to_string());
    }
    let mut parser = ExprParser::new(tokens);
    let expr = parser.parse_expr()?;
    if parser.pos < parser.tokens.len() {
        return Err(format!(
            "Unexpected trailing tokens in expression: {:?}",
            &parser.tokens[parser.pos..]
        ));
    }
    Ok(fold_constants(expr))
}

fn fold_constants(expr: Expr) -> Expr {
    match expr {
        Expr::BinOp { op, left, right } => {
            let l = fold_constants(*left);
            let r = fold_constants(*right);
            if let (Expr::Int(lv), Expr::Int(rv)) = (&l, &r) {
                let folded = match op {
                    Op::Add => Some(lv.wrapping_add(*rv)),
                    Op::Sub => Some(lv.wrapping_sub(*rv)),
                    Op::Mult => Some(lv.wrapping_mul(*rv)),
                    Op::Pow => {
                        if *rv >= 0 && *rv <= 62 {
                            lv.checked_pow(*rv as u32)
                        } else {
                            None
                        }
                    }
                    Op::Div => {
                        if *rv != 0 {
                            Some(lv / rv)
                        } else {
                            None
                        }
                    }
                    Op::Mod => {
                        if *rv != 0 {
                            Some(lv % rv)
                        } else {
                            None
                        }
                    }
                    Op::Lshift => Some(lv.wrapping_shl(*rv as u32)),
                    Op::Rshift => Some(lv.wrapping_shr(*rv as u32)),
                    Op::Bitor => Some(lv | rv),
                    Op::Bitxor => Some(lv ^ rv),
                    Op::Bitand => Some(lv & rv),
                    Op::Eq => Some(i64::from(lv == rv)),
                    Op::Noteq => Some(i64::from(lv != rv)),
                    Op::Lt => Some(i64::from(lv < rv)),
                    Op::Lte => Some(i64::from(lv <= rv)),
                    Op::Gt => Some(i64::from(lv > rv)),
                    Op::Gte => Some(i64::from(lv >= rv)),
                    _ => None,
                };
                if let Some(val) = folded {
                    return Expr::Int(val);
                }
            }
            Expr::BinOp {
                op,
                left: Box::new(l),
                right: Box::new(r),
            }
        }
        Expr::BoolOp { op, values } => {
            let folded_vals: Vec<Expr> = values.into_iter().map(fold_constants).collect();
            if folded_vals.iter().all(|v| matches!(v, Expr::Int(_))) {
                let ints: Vec<i64> = folded_vals
                    .iter()
                    .filter_map(|v| if let Expr::Int(n) = v { Some(*n) } else { None })
                    .collect();
                let res = match op {
                    Op::Logiand => i64::from(ints.iter().all(|&x| x != 0)),
                    Op::Logior => i64::from(ints.iter().any(|&x| x != 0)),
                    _ => 0,
                };
                return Expr::Int(res);
            }
            Expr::BoolOp {
                op,
                values: folded_vals,
            }
        }
        Expr::UnaryOp { op, operand } => Expr::UnaryOp {
            op,
            operand: Box::new(fold_constants(*operand)),
        },
        Expr::Call { name, args } => Expr::Call {
            name,
            args: args.into_iter().map(fold_constants).collect(),
        },
        other => other,
    }
}

// ---------------- Transpiler & Block Builder ----------------

fn parse_statements(lines: &[DsLine], pos: &mut usize) -> Result<Vec<Stmt>, CompileResult> {
    let mut stmts = Vec::new();
    while *pos < lines.len() {
        let line_obj = &lines[*pos];
        let lnum = line_obj.orig_lnum_sf1;
        let lnum_usize = lnum.unsigned_abs();
        let raw = line_obj.content.trim_start_matches([' ', '\t']);
        if raw.is_empty() {
            *pos += 1;
            continue;
        }
        let first_word = raw.split_whitespace().next().unwrap_or("");

        if matches!(first_word, "END_IF" | "ELSE" | "END_WHILE" | "END_FUN")
            || raw.starts_with("ELSE IF ")
        {
            break;
        }

        *pos += 1;

        if first_word == "IF" {
            let cond_str = raw
                .strip_prefix("IF")
                .unwrap_or("")
                .trim()
                .strip_suffix("THEN")
                .unwrap_or_else(|| raw.strip_prefix("IF").unwrap_or("").trim())
                .trim();
            let cond_op = replace_operators(cond_str);
            let test = parse_expression_str(&cond_op)
                .map_err(|e| CompileResult::err(lnum_usize, raw, e))?;
            let mut body = parse_statements(lines, pos)?;
            if body.is_empty() {
                body.push(Stmt::Pass {
                    orig_lnum: lnum,
                    orig_line: raw.to_string(),
                });
            }
            let orelse = parse_if_tail(lines, pos)?;
            stmts.push(Stmt::If {
                test,
                body,
                orelse,
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else if first_word == "WHILE" {
            let cond_str = raw.strip_prefix("WHILE").unwrap_or("").trim();
            let cond_op = replace_operators(cond_str);
            let test = parse_expression_str(&cond_op)
                .map_err(|e| CompileResult::err(lnum_usize, raw, e))?;
            let mut body = parse_statements(lines, pos)?;
            if body.is_empty() {
                body.push(Stmt::Pass {
                    orig_lnum: lnum,
                    orig_line: raw.to_string(),
                });
            }
            if *pos < lines.len()
                && lines[*pos].content.split_whitespace().next() == Some("END_WHILE")
            {
                *pos += 1;
            }
            stmts.push(Stmt::While {
                test,
                body,
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else if first_word == "FUN" {
            let header = raw.strip_prefix("FUN").unwrap_or("").trim();
            let Some((fn_name, rest)) = header.split_once('(') else {
                return Err(CompileResult::err(lnum_usize, raw, "Invalid FUN header"));
            };
            let args_str = rest.strip_suffix(')').unwrap_or(rest);
            let args: Vec<String> = args_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let mut body = parse_statements(lines, pos)?;
            if body.is_empty() {
                body.push(Stmt::Pass {
                    orig_lnum: lnum,
                    orig_line: raw.to_string(),
                });
            }
            if *pos < lines.len()
                && lines[*pos].content.split_whitespace().next() == Some("END_FUN")
            {
                *pos += 1;
            }
            stmts.push(Stmt::FuncDef {
                name: fn_name.trim().to_string(),
                args,
                body,
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else if first_word == "CONTINUE" {
            stmts.push(Stmt::Continue {
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else if first_word == "LBREAK" {
            stmts.push(Stmt::Break {
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else if first_word == "RETURN" {
            let expr_str = raw.strip_prefix("RETURN").unwrap_or("").trim();
            let value = if expr_str.is_empty() {
                None
            } else {
                let op_str = replace_operators(expr_str);
                Some(
                    parse_expression_str(&op_str)
                        .map_err(|e| CompileResult::err(lnum_usize, raw, e))?,
                )
            };
            stmts.push(Stmt::Return {
                value,
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else if first_word == "pass" {
            stmts.push(Stmt::Pass {
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else if first_word == "OLED_LPRINT" || first_word == "OLED_CPRINT" {
            let opt = if first_word == "OLED_CPRINT" { 1 } else { 0 };
            let str_content = if raw.len() > first_word.len() + 1 {
                &raw[first_word.len() + 1..]
            } else {
                ""
            };
            stmts.push(Stmt::Assign {
                target: DUMMY_VAR_NAME.to_string(),
                value: Expr::Call {
                    name: "OLED_OPTPRINT".to_string(),
                    args: vec![Expr::Int(opt), Expr::Str(str_content.to_string())],
                },
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else if is_str_or_keypress_cmd(first_word) {
            let str_content = if raw.len() > first_word.len() + 1 {
                &raw[first_word.len() + 1..]
            } else {
                ""
            };
            stmts.push(Stmt::Assign {
                target: DUMMY_VAR_NAME.to_string(),
                value: Expr::Call {
                    name: first_word.to_string(),
                    args: vec![Expr::Str(str_content.to_string())],
                },
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else if let Some(binfo) = builtin_func_lookup(first_word) {
            // Check if called in command style `DELAY 100` vs expression style `DELAY(100)`
            let after_kw = raw[first_word.len()..].trim();
            let call_expr_str = if after_kw.starts_with('(') && after_kw.ends_with(')') {
                replace_operators(raw)
            } else {
                let args: Vec<&str> = after_kw.split(' ').filter(|s| !s.is_empty()).collect();
                let joined = args.join(", ");
                replace_operators(&format!("{first_word}({joined})"))
            };
            let _ = binfo;
            let expr = parse_expression_str(&call_expr_str)
                .map_err(|e| CompileResult::err(lnum_usize, raw, e))?;
            stmts.push(Stmt::Assign {
                target: DUMMY_VAR_NAME.to_string(),
                value: expr,
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        } else {
            let mut stmt_str = if first_word == "VAR" {
                raw[3..].trim().to_string()
            } else {
                raw.to_string()
            };
            stmt_str = replace_operators(&stmt_str);

            // Check augmented assignment (+=, -=, *=, /=, %=, <<=, >>=, |=, ^=, &=)
            let aug_ops: &[(&str, Op)] = &[
                ("<<=", Op::Lshift),
                (">>=", Op::Rshift),
                ("+=", Op::Add),
                ("-=", Op::Sub),
                ("*=", Op::Mult),
                ("/=", Op::Div),
                ("%=", Op::Mod),
                ("|=", Op::Bitor),
                ("^=", Op::Bitxor),
                ("&=", Op::Bitand),
            ];
            let mut matched_aug = None;
            for &(sym, op) in aug_ops {
                if let Some((lhs, rhs)) = stmt_str.split_once(sym) {
                    let lhs_trim = lhs.trim();
                    if !lhs_trim.is_empty()
                        && lhs_trim
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        matched_aug = Some((lhs_trim.to_string(), op, rhs.trim().to_string()));
                        break;
                    }
                }
            }

            if let Some((target, op, rhs_str)) = matched_aug {
                let value = parse_expression_str(&rhs_str)
                    .map_err(|e| CompileResult::err(lnum_usize, raw, e))?;
                stmts.push(Stmt::AugAssign {
                    target,
                    op,
                    value,
                    orig_lnum: lnum,
                    orig_line: raw.to_string(),
                });
            } else if let Some((lhs, rhs)) = find_top_level_assign(&stmt_str) {
                let target = lhs.trim().to_string();
                let value = parse_expression_str(rhs.trim())
                    .map_err(|e| CompileResult::err(lnum_usize, raw, e))?;
                stmts.push(Stmt::Assign {
                    target,
                    value,
                    orig_lnum: lnum,
                    orig_line: raw.to_string(),
                });
            } else {
                let expr = parse_expression_str(&stmt_str)
                    .map_err(|e| CompileResult::err(lnum_usize, raw, e))?;
                stmts.push(Stmt::Assign {
                    target: DUMMY_VAR_NAME.to_string(),
                    value: expr,
                    orig_lnum: lnum,
                    orig_line: raw.to_string(),
                });
            }
        }
    }
    Ok(stmts)
}

fn parse_if_tail(lines: &[DsLine], pos: &mut usize) -> Result<Vec<Stmt>, CompileResult> {
    if *pos >= lines.len() {
        return Ok(Vec::new());
    }
    let line_obj = &lines[*pos];
    let lnum = line_obj.orig_lnum_sf1;
    let lnum_usize = lnum.unsigned_abs();
    let raw = line_obj.content.trim_start_matches([' ', '\t']);
    let first_word = raw.split_whitespace().next().unwrap_or("");

    if raw.starts_with("ELSE IF ") {
        *pos += 1;
        let cond_str = raw
            .strip_prefix("ELSE IF")
            .unwrap_or("")
            .trim()
            .strip_suffix("THEN")
            .unwrap_or_else(|| raw.strip_prefix("ELSE IF").unwrap_or("").trim())
            .trim();
        let cond_op = replace_operators(cond_str);
        let test =
            parse_expression_str(&cond_op).map_err(|e| CompileResult::err(lnum_usize, raw, e))?;
        let mut body = parse_statements(lines, pos)?;
        if body.is_empty() {
            body.push(Stmt::Pass {
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        }
        let orelse = parse_if_tail(lines, pos)?;
        Ok(vec![Stmt::If {
            test,
            body,
            orelse,
            orig_lnum: lnum,
            orig_line: raw.to_string(),
        }])
    } else if first_word == "ELSE" {
        *pos += 1;
        let mut body = parse_statements(lines, pos)?;
        if body.is_empty() {
            body.push(Stmt::Pass {
                orig_lnum: lnum,
                orig_line: raw.to_string(),
            });
        }
        if *pos < lines.len() && lines[*pos].content.split_whitespace().next() == Some("END_IF") {
            *pos += 1;
        }
        Ok(body)
    } else if first_word == "END_IF" {
        *pos += 1;
        Ok(Vec::new())
    } else {
        Ok(Vec::new())
    }
}

fn find_top_level_assign(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut depth = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'(' {
            depth += 1;
        } else if b == b')' {
            depth = depth.saturating_sub(1);
        } else if b == b'=' && depth == 0 {
            let prev = if i > 0 { bytes[i - 1] } else { 0 };
            let next = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
            if next != b'=' && !matches!(prev, b'=' | b'!' | b'<' | b'>') {
                return Some((&s[..i], &s[i + 1..]));
            }
        }
        i += 1;
    }
    None
}

// ---------------- Dead Function Elimination & Code Generation ----------------

fn collect_calls_in_expr(expr: &Expr, out: &mut Vec<String>) {
    match expr {
        Expr::Call { name, args } => {
            out.push(name.clone());
            for a in args {
                collect_calls_in_expr(a, out);
            }
        }
        Expr::BinOp { left, right, .. } => {
            collect_calls_in_expr(left, out);
            collect_calls_in_expr(right, out);
        }
        Expr::BoolOp { values, .. } => {
            for v in values {
                collect_calls_in_expr(v, out);
            }
        }
        Expr::UnaryOp { operand, .. } => collect_calls_in_expr(operand, out),
        _ => {}
    }
}

fn collect_calls_in_stmts(stmts: &[Stmt], out: &mut Vec<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Assign { value, .. } | Stmt::AugAssign { value, .. } => {
                collect_calls_in_expr(value, out);
            }
            Stmt::If {
                test, body, orelse, ..
            } => {
                collect_calls_in_expr(test, out);
                collect_calls_in_stmts(body, out);
                collect_calls_in_stmts(orelse, out);
            }
            Stmt::While { test, body, .. } => {
                collect_calls_in_expr(test, out);
                collect_calls_in_stmts(body, out);
            }
            Stmt::Return {
                value: Some(v), ..
            } => collect_calls_in_expr(v, out),
            _ => {}
        }
    }
}

fn filter_reachable_functions(stmts: Vec<Stmt>, remove_unused_func: bool) -> Vec<Stmt> {
    if !remove_unused_func {
        return stmts;
    }
    let mut defined_funcs: HashMap<String, &Stmt> = HashMap::new();
    let mut worklist: Vec<&Stmt> = Vec::new();
    for s in &stmts {
        if let Stmt::FuncDef { name, .. } = s {
            defined_funcs.insert(name.clone(), s);
        } else {
            worklist.push(s);
        }
    }

    let mut live_funcs: HashSet<String> = HashSet::new();
    while let Some(node) = worklist.pop() {
        let mut calls = Vec::new();
        match node {
            Stmt::FuncDef { body, .. } => collect_calls_in_stmts(body, &mut calls),
            other => collect_calls_in_stmts(std::slice::from_ref(other), &mut calls),
        }
        for cname in calls {
            if let Some(&fn_stmt) = defined_funcs.get(&cname) {
                if live_funcs.insert(cname) {
                    worklist.push(fn_stmt);
                }
            }
        }
    }

    stmts
        .into_iter()
        .filter(|s| match s {
            Stmt::FuncDef { name, .. } => live_funcs.contains(name),
            _ => true,
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct VarInfo {
    name: String,
    sym_type: SymType,
    func: Option<String>,
}

struct CodegenCtx {
    root_assembly_list: Vec<Instruction>,
    func_assembly_order: Vec<String>,
    func_assembly_dict: HashMap<String, Vec<Instruction>>,
    func_args_dict: HashMap<String, Vec<String>>,
    user_declared_vars: HashMap<Option<String>, HashSet<String>>,
    reserved_vars: HashMap<&'static str, u16>,
    var_info_set: BTreeSet<VarInfo>,
    current_function: Option<String>,
    while_label_stack: Vec<(String, String)>,
}

impl CodegenCtx {
    fn active_list(&mut self) -> &mut Vec<Instruction> {
        if let Some(ref fname) = self.current_function {
            if !self.func_assembly_dict.contains_key(fname) {
                self.func_assembly_order.push(fname.clone());
                self.func_assembly_dict.insert(fname.clone(), Vec::new());
            }
            self.func_assembly_dict.get_mut(fname).unwrap()
        } else {
            &mut self.root_assembly_list
        }
    }

    fn classify_name(&self, name: &str) -> Result<SymType, String> {
        if self.reserved_vars.contains_key(name) {
            return Ok(SymType::ReservedVar);
        }
        if let Some(ref cur_fn) = self.current_function {
            let is_param = self
                .func_args_dict
                .get(cur_fn)
                .map_or(false, |args| args.iter().any(|a| a == name));
            let is_local = self
                .user_declared_vars
                .get(&Some(cur_fn.clone()))
                .map_or(false, |set| set.contains(name));
            if is_param && is_local {
                return Err(format!("Variable \"{name}\" cannot be both arg and local"));
            }
            if is_param {
                return Ok(SymType::FuncArg);
            }
            if is_local {
                return Ok(SymType::FuncLocalVar);
            }
        }
        let is_global = self
            .user_declared_vars
            .get(&None)
            .map_or(false, |set| set.contains(name));
        if is_global {
            return Ok(SymType::GlobalVar);
        }
        let suffix = self
            .current_function
            .as_ref()
            .map(|f| format!(" in function \"{f}()\""))
            .unwrap_or_default();
        Err(format!("Unknown symbol \"{name}\"{suffix}"))
    }

    fn emit_var_access(&mut self, name: &str, is_store: bool, comment: &str) -> Result<(), String> {
        let sym_type = self.classify_name(name)?;
        let parent_func = match sym_type {
            SymType::GlobalVar | SymType::ReservedVar => None,
            SymType::FuncArg | SymType::FuncLocalVar => self.current_function.clone(),
        };
        self.var_info_set.insert(VarInfo {
            name: name.to_string(),
            sym_type,
            func: parent_func,
        });

        let opcode = if is_store {
            if matches!(sym_type, SymType::FuncArg | SymType::FuncLocalVar) {
                Op::Popr
            } else {
                Op::Popi
            }
        } else if matches!(sym_type, SymType::FuncArg | SymType::FuncLocalVar) {
            Op::Pushr
        } else {
            Op::Pushi
        };

        let cur_fn = self.current_function.clone();
        self.active_list().push(Instruction {
            opcode,
            payload: Payload::Sym(name.to_string()),
            label: None,
            comment: comment.to_string(),
            addr: 0,
            parent_func: cur_fn,
            var_type: Some(sym_type),
        });
        Ok(())
    }

    fn emit_pushc(&mut self, value: i64, comment: &str) {
        let val = (value as u32) as i64;
        let opcode = if val > 0xFFFF {
            Op::Pushc32
        } else if val > 0xFF {
            Op::Pushc16
        } else {
            Op::Pushc8
        };
        self.active_list().push(Instruction {
            opcode,
            payload: Payload::Int(val),
            label: None,
            comment: comment.to_string(),
            addr: 0,
            parent_func: None,
            var_type: None,
        });
    }

    fn walk_expr(
        &mut self,
        expr: &Expr,
        caller_func: Option<&str>,
        comment: &str,
    ) -> Result<(), String> {
        match expr {
            Expr::Var(name) => self.emit_var_access(name, false, comment)?,
            Expr::Int(v) => self.emit_pushc(*v, comment),
            Expr::Str(s) => {
                if let Some(cf) = caller_func {
                    if let Some(binfo) = builtin_func_lookup(cf) {
                        if binfo.is_str_func {
                            let cur_fn = self.current_function.clone();
                            self.active_list().push(Instruction {
                                opcode: Op::Pushstr,
                                payload: Payload::Sym(s.clone()),
                                label: None,
                                comment: comment.to_string(),
                                addr: 0,
                                parent_func: cur_fn,
                                var_type: None,
                            });
                            return Ok(());
                        }
                        if binfo.is_keypress_func {
                            let kv = get_key_combined_value(s)?;
                            self.active_list().push(Instruction {
                                opcode: Op::Pushc16,
                                payload: Payload::Int(kv as i64),
                                label: None,
                                comment: comment.to_string(),
                                addr: 0,
                                parent_func: None,
                                var_type: None,
                            });
                            return Ok(());
                        }
                    }
                }
                if s.len() == 1 {
                    self.emit_pushc(s.as_bytes()[0] as i64, comment);
                } else {
                    return Err(format!("Unsupported Constant: {s}"));
                }
            }
            Expr::BinOp { op, left, right } => {
                // Push args right-to-left
                self.walk_expr(right, caller_func, comment)?;
                self.walk_expr(left, caller_func, comment)?;
                self.active_list().push(Instruction {
                    opcode: *op,
                    payload: Payload::None,
                    label: None,
                    comment: comment.to_string(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
            }
            Expr::BoolOp { op, values } => {
                for item in values.iter().rev() {
                    self.walk_expr(item, caller_func, comment)?;
                }
                for _ in 0..(values.len().saturating_sub(1)) {
                    self.active_list().push(Instruction {
                        opcode: *op,
                        payload: Payload::None,
                        label: None,
                        comment: comment.to_string(),
                        addr: 0,
                        parent_func: None,
                        var_type: None,
                    });
                }
            }
            Expr::UnaryOp { op, operand } => {
                self.walk_expr(operand, caller_func, comment)?;
                self.active_list().push(Instruction {
                    opcode: *op,
                    payload: Payload::None,
                    label: None,
                    comment: comment.to_string(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
            }
            Expr::Call { name, args } => {
                let expected_args = if let Some(binfo) = builtin_func_lookup(name) {
                    binfo.arg_len
                } else if let Some(fn_args) = self.func_args_dict.get(name) {
                    fn_args.len()
                } else {
                    return Err(format!("Function {name}() not found"));
                };
                if args.len() != expected_args {
                    return Err("Wrong number of arguments".to_string());
                }
                for arg in args.iter().rev() {
                    self.walk_expr(arg, Some(name.as_str()), comment)?;
                }
                if let Some(binfo) = builtin_func_lookup(name) {
                    self.active_list().push(Instruction {
                        opcode: binfo.opcode,
                        payload: Payload::None,
                        label: None,
                        comment: comment.to_string(),
                        addr: 0,
                        parent_func: None,
                        var_type: None,
                    });
                    if !binfo.has_return_value {
                        self.active_list().push(Instruction {
                            opcode: Op::Push0,
                            payload: Payload::None,
                            label: None,
                            comment: comment.to_string(),
                            addr: 0,
                            parent_func: None,
                            var_type: None,
                        });
                    }
                } else {
                    self.active_list().push(Instruction {
                        opcode: Op::Call,
                        payload: Payload::Sym(format!("func_{name}")),
                        label: None,
                        comment: comment.to_string(),
                        addr: 0,
                        parent_func: None,
                        var_type: None,
                    });
                }
            }
        }
        Ok(())
    }

    fn walk_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileResult> {
        match stmt {
            Stmt::Assign {
                target,
                value,
                orig_lnum,
                orig_line,
            } => {
                self.walk_expr(value, None, orig_line)
                    .map_err(|e| CompileResult::err(orig_lnum.unsigned_abs(), orig_line, e))?;
                self.emit_var_access(target, true, orig_line)
                    .map_err(|e| CompileResult::err(orig_lnum.unsigned_abs(), orig_line, e))?;
            }
            Stmt::AugAssign {
                target,
                op,
                value,
                orig_lnum,
                orig_line,
            } => {
                self.walk_expr(value, None, orig_line)
                    .map_err(|e| CompileResult::err(orig_lnum.unsigned_abs(), orig_line, e))?;
                self.emit_var_access(target, false, orig_line)
                    .map_err(|e| CompileResult::err(orig_lnum.unsigned_abs(), orig_line, e))?;
                self.active_list().push(Instruction {
                    opcode: *op,
                    payload: Payload::None,
                    label: None,
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
                self.emit_var_access(target, true, orig_line)
                    .map_err(|e| CompileResult::err(orig_lnum.unsigned_abs(), orig_line, e))?;
            }
            Stmt::If {
                test,
                body,
                orelse,
                orig_lnum,
                orig_line,
            } => {
                let mut if_skip_label = format!("If_skip@{orig_lnum}");
                let if_end_label = format!("If_end@{orig_lnum}");
                if orelse.is_empty() {
                    if_skip_label = if_end_label.clone();
                }
                self.walk_expr(test, None, orig_line)
                    .map_err(|e| CompileResult::err(orig_lnum.unsigned_abs(), orig_line, e))?;
                self.active_list().push(Instruction {
                    opcode: Op::Brz,
                    payload: Payload::Sym(if_skip_label.clone()),
                    label: None,
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
                for s in body {
                    self.walk_stmt(s)?;
                }
                if !orelse.is_empty() {
                    self.active_list().push(Instruction {
                        opcode: Op::Jmp,
                        payload: Payload::Sym(if_end_label.clone()),
                        label: None,
                        comment: orig_line.clone(),
                        addr: 0,
                        parent_func: None,
                        var_type: None,
                    });
                    self.active_list().push(Instruction {
                        opcode: Op::Nop,
                        payload: Payload::None,
                        label: Some(if_skip_label),
                        comment: orig_line.clone(),
                        addr: 0,
                        parent_func: None,
                        var_type: None,
                    });
                    for s in orelse {
                        self.walk_stmt(s)?;
                    }
                }
                self.active_list().push(Instruction {
                    opcode: Op::Nop,
                    payload: Payload::None,
                    label: Some(if_end_label),
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
            }
            Stmt::While {
                test,
                body,
                orig_lnum,
                orig_line,
            } => {
                let while_start_label = format!("While_start@{orig_lnum}");
                let while_end_label = format!("While_end@{orig_lnum}");
                self.active_list().push(Instruction {
                    opcode: Op::Nop,
                    payload: Payload::None,
                    label: Some(while_start_label.clone()),
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
                self.walk_expr(test, None, orig_line)
                    .map_err(|e| CompileResult::err(orig_lnum.unsigned_abs(), orig_line, e))?;
                self.active_list().push(Instruction {
                    opcode: Op::Brz,
                    payload: Payload::Sym(while_end_label.clone()),
                    label: None,
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
                self.while_label_stack
                    .push((while_start_label.clone(), while_end_label.clone()));
                for s in body {
                    self.walk_stmt(s)?;
                }
                self.while_label_stack.pop();
                self.active_list().push(Instruction {
                    opcode: Op::Jmp,
                    payload: Payload::Sym(while_start_label),
                    label: None,
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
                self.active_list().push(Instruction {
                    opcode: Op::Nop,
                    payload: Payload::None,
                    label: Some(while_end_label),
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
            }
            Stmt::FuncDef {
                name,
                args,
                body,
                orig_line,
                ..
            } => {
                let prev_fn = self.current_function.take();
                self.current_function = Some(name.clone());
                let func_label = format!("func_{name}");
                self.active_list().push(Instruction {
                    opcode: Op::Nop,
                    payload: Payload::None,
                    label: Some(func_label),
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
                self.active_list().push(Instruction {
                    opcode: Op::Alloc,
                    payload: Payload::Sym(name.clone()),
                    label: None,
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
                for s in body {
                    self.walk_stmt(s)?;
                }
                let ends_with_ret = self
                    .active_list()
                    .last()
                    .map_or(false, |i| i.opcode == Op::Ret);
                if !ends_with_ret {
                    self.active_list().push(Instruction {
                        opcode: Op::Push0,
                        payload: Payload::None,
                        label: None,
                        comment: orig_line.clone(),
                        addr: 0,
                        parent_func: None,
                        var_type: None,
                    });
                    self.active_list().push(Instruction {
                        opcode: Op::Ret,
                        payload: Payload::Int(args.len() as i64),
                        label: None,
                        comment: orig_line.clone(),
                        addr: 0,
                        parent_func: None,
                        var_type: None,
                    });
                }
                self.current_function = prev_fn;
            }
            Stmt::Return {
                value,
                orig_lnum,
                orig_line,
            } => {
                if let Some(expr) = value {
                    self.walk_expr(expr, None, orig_line)
                        .map_err(|e| CompileResult::err(orig_lnum.unsigned_abs(), orig_line, e))?;
                } else {
                    self.active_list().push(Instruction {
                        opcode: Op::Push0,
                        payload: Payload::None,
                        label: None,
                        comment: orig_line.clone(),
                        addr: 0,
                        parent_func: None,
                        var_type: None,
                    });
                }
                let arg_count = self
                    .current_function
                    .as_ref()
                    .and_then(|f| self.func_args_dict.get(f))
                    .map_or(0, |a| a.len());
                self.active_list().push(Instruction {
                    opcode: Op::Ret,
                    payload: Payload::Int(arg_count as i64),
                    label: None,
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
            }
            Stmt::Break {
                orig_lnum,
                orig_line,
            } => {
                let Some((_, end_lbl)) = self.while_label_stack.last().cloned() else {
                    return Err(CompileResult::err(
                        orig_lnum.unsigned_abs(),
                        orig_line,
                        "BREAK outside WHILE",
                    ));
                };
                self.active_list().push(Instruction {
                    opcode: Op::Jmp,
                    payload: Payload::Sym(end_lbl),
                    label: None,
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
            }
            Stmt::Continue {
                orig_lnum,
                orig_line,
            } => {
                let Some((start_lbl, _)) = self.while_label_stack.last().cloned() else {
                    return Err(CompileResult::err(
                        orig_lnum.unsigned_abs(),
                        orig_line,
                        "CONTINUE outside WHILE",
                    ));
                };
                self.active_list().push(Instruction {
                    opcode: Op::Jmp,
                    payload: Payload::Sym(start_lbl),
                    label: None,
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
            }
            Stmt::Pass { orig_line, .. } => {
                self.active_list().push(Instruction {
                    opcode: Op::Nop,
                    payload: Payload::None,
                    label: None,
                    comment: orig_line.clone(),
                    addr: 0,
                    parent_func: None,
                    var_type: None,
                });
            }
        }
        Ok(())
    }
}

fn replace_dummy_with_drop(list: &mut [Instruction]) {
    for inst in list {
        if inst.opcode == Op::Popi {
            if let Payload::Sym(ref s) = inst.payload {
                if s == DUMMY_VAR_NAME {
                    inst.opcode = Op::Drop;
                    inst.payload = Payload::None;
                }
            }
        }
    }
}

fn optimize_pass(
    instruction_list: Vec<Instruction>,
    func_vars: &HashMap<String, (Vec<String>, Vec<String>)>,
) -> Vec<Instruction> {
    let mut out = Vec::with_capacity(instruction_list.len());
    let mut i = 0;
    while i < instruction_list.len() {
        let cur = &instruction_list[i];
        if i + 1 < instruction_list.len() {
            let next = &instruction_list[i + 1];
            if cur.opcode == Op::Push0 && next.opcode == Op::Drop {
                i += 2;
                continue;
            }
            let is_same_save_load = (cur.opcode == Op::Popi && next.opcode == Op::Pushi)
                || (cur.opcode == Op::Popr && next.opcode == Op::Pushr);
            if is_same_save_load && cur.payload == next.payload {
                out.push(Instruction::new(Op::Dup));
                out.push(cur.clone());
                i += 2;
                continue;
            }
        }
        if cur.opcode.is_pushc() && cur.payload == Payload::Int(0) {
            let mut inst = Instruction::new(Op::Push0);
            inst.label = cur.label.clone();
            inst.comment = cur.comment.clone();
            out.push(inst);
            i += 1;
            continue;
        }
        if cur.opcode.is_pushc() && cur.payload == Payload::Int(1) {
            let mut inst = Instruction::new(Op::Push1);
            inst.label = cur.label.clone();
            inst.comment = cur.comment.clone();
            out.push(inst);
            i += 1;
            continue;
        }
        if cur.opcode == Op::Alloc {
            if let Payload::Sym(ref fn_name) = cur.payload {
                if let Some((_, locals)) = func_vars.get(fn_name) {
                    if locals.is_empty() {
                        i += 1;
                        continue;
                    }
                } else {
                    i += 1;
                    continue;
                }
            }
        }
        out.push(cur.clone());
        i += 1;
    }
    out
}

fn extract_printf_specifier(text: &str) -> &str {
    let bytes = text.as_bytes();
    if bytes.first() != Some(&b'%') {
        return "";
    }
    let mut i = 1;
    while i < bytes.len() && matches!(bytes[i], b'-' | b'+' | b' ' | b'#' | b'0') {
        i += 1;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i < bytes.len() && matches!(bytes[i], b'd' | b'u' | b'x' | b'X') {
        &text[..=i]
    } else {
        ""
    }
}

fn replace_var_in_str(
    msg: &str,
    parent_func: Option<&str>,
    reserved: &HashMap<&'static str, u16>,
    func_vars: &HashMap<String, (Vec<String>, Vec<String>)>,
    udgv_lookup: &BTreeMap<String, u16>,
) -> Vec<u8> {
    let mut result = Vec::with_capacity(msg.len() + 4);
    let chars: Vec<(usize, char)> = msg.char_indices().collect();
    let mut idx = 0;

    while idx < chars.len() {
        let (byte_pos, ch) = chars[idx];
        if ch == '$' {
            let after_dollar = &msg[byte_pos + 1..];
            let mut matched: Option<(&str, i32, SymType)> = None;
            for (end_byte, end_ch) in after_dollar.char_indices().rev() {
                let candidate = &after_dollar[..end_byte + end_ch.len_utf8()];
                if let Some(&addr) = reserved.get(candidate) {
                    matched = Some((candidate, addr as i32, SymType::ReservedVar));
                    break;
                }
                if let Some(pf) = parent_func {
                    if let Some((args, locals)) = func_vars.get(pf) {
                        if let Some(pos) = args.iter().position(|a| a == candidate) {
                            matched = Some((candidate, ((pos + 1) * 4) as i32, SymType::FuncArg));
                            break;
                        }
                        if let Some(pos) = locals.iter().position(|l| l == candidate) {
                            matched =
                                Some((candidate, -(((pos + 1) * 4) as i32), SymType::FuncLocalVar));
                            break;
                        }
                    }
                }
                if let Some(&addr) = udgv_lookup.get(candidate) {
                    matched = Some((candidate, addr as i32, SymType::GlobalVar));
                    break;
                }
            }

            if let Some((var_name, var_addr, sym_type)) = matched {
                let (boundary, addr_bytes) =
                    if matches!(sym_type, SymType::FuncArg | SymType::FuncLocalVar) {
                        (VAR_BOUNDARY_FP_REL, (var_addr as i16).to_le_bytes())
                    } else {
                        (VAR_BOUNDARY_UDGV, (var_addr as u16).to_le_bytes())
                    };
                let after_var = &after_dollar[var_name.len()..];
                let spec = extract_printf_specifier(after_var);

                result.push(boundary);
                result.extend_from_slice(&addr_bytes);
                result.extend_from_slice(spec.as_bytes());
                result.push(boundary);

                let skip_chars = 1 + var_name.chars().count() + spec.chars().count();
                idx += skip_chars;
                continue;
            }
        }
        let mut buf = [0u8; 4];
        result.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
        idx += 1;
    }
    result.push(0);
    result
}

pub fn compile_duckyscript_to_dsb(
    script_text: &str,
    allow_abort: bool,
    disable_autorepeat: bool,
) -> CompileResult {
    let mut program_listing = Vec::new();
    if allow_abort {
        program_listing.push(DsLine {
            content: "$_ALLOW_ABORT = 1".to_string(),
            orig_lnum_sf1: 1,
            indent_level: 0,
        });
    }
    if disable_autorepeat {
        program_listing.push(DsLine {
            content: "$_DONT_REPEAT = 1".to_string(),
            orig_lnum_sf1: 1,
            indent_level: 0,
        });
    }
    for (i, line) in script_text
        .trim_start_matches([' ', '\t'])
        .split('\n')
        .enumerate()
    {
        program_listing.push(DsLine {
            content: line.trim_end_matches('\r').to_string(),
            orig_lnum_sf1: (i + 1) as isize,
            indent_level: 0,
        });
    }

    let stdlib_raw: Vec<DsLine> = default_stdlib_lines()
        .into_iter()
        .enumerate()
        .map(|(i, content)| DsLine {
            content,
            orig_lnum_sf1: -((i + 1) as isize),
            indent_level: 0,
        })
        .collect();
    let stdlib_preprocessed = preprocess_all(stdlib_raw, None).ok().map(|r| r.lines);

    let preproc = match preprocess_all(program_listing, stdlib_preprocessed) {
        Ok(r) => r,
        Err(err) => return err,
    };

    let mut pos = 0;
    let raw_stmts = match parse_statements(&preproc.lines, &mut pos) {
        Ok(s) => s,
        Err(err) => return err,
    };

    let live_stmts = filter_reachable_functions(raw_stmts, true);

    let mut func_args_dict: HashMap<String, Vec<String>> = HashMap::new();
    for s in &live_stmts {
        if let Stmt::FuncDef { name, args, .. } = s {
            func_args_dict.insert(name.clone(), args.clone());
        }
    }

    let mut ctx = CodegenCtx {
        root_assembly_list: vec![Instruction {
            opcode: Op::Vmver,
            payload: Payload::Int(DSVM_VERSION as i64),
            label: None,
            comment: String::new(),
            addr: 0,
            parent_func: None,
            var_type: None,
        }],
        func_assembly_order: Vec::new(),
        func_assembly_dict: HashMap::new(),
        func_args_dict: func_args_dict.clone(),
        user_declared_vars: preproc.user_declared_vars,
        reserved_vars: reserved_variables_map(),
        var_info_set: BTreeSet::new(),
        current_function: None,
        while_label_stack: Vec::new(),
    };

    for stmt in &live_stmts {
        if let Err(err) = ctx.walk_stmt(stmt) {
            return err;
        }
    }

    // Group function args and sorted local variables
    let mut func_vars: HashMap<String, (Vec<String>, Vec<String>)> = HashMap::new();
    for (fn_name, args) in &func_args_dict {
        func_vars.insert(fn_name.clone(), (args.clone(), Vec::new()));
    }
    for vinfo in &ctx.var_info_set {
        if vinfo.sym_type == SymType::FuncLocalVar {
            if let Some(ref fn_name) = vinfo.func {
                let entry = func_vars
                    .entry(fn_name.clone())
                    .or_insert_with(|| (Vec::new(), Vec::new()));
                if !entry.1.contains(&vinfo.name) {
                    entry.1.push(vinfo.name.clone());
                }
            }
        }
    }
    for (_, (_, locals)) in func_vars.iter_mut() {
        locals.sort();
    }

    replace_dummy_with_drop(&mut ctx.root_assembly_list);
    for list in ctx.func_assembly_dict.values_mut() {
        replace_dummy_with_drop(list);
    }

    ctx.root_assembly_list = optimize_pass(ctx.root_assembly_list, &func_vars);
    for fn_name in &ctx.func_assembly_order {
        if let Some(list) = ctx.func_assembly_dict.remove(fn_name) {
            ctx.func_assembly_dict
                .insert(fn_name.clone(), optimize_pass(list, &func_vars));
        }
    }
    ctx.root_assembly_list.push(Instruction::new(Op::Halt));

    // Global variables sorted alphabetically
    let mut udgv_lookup: BTreeMap<String, u16> = BTreeMap::new();
    let global_names: BTreeSet<String> = ctx
        .var_info_set
        .iter()
        .filter(|v| v.sym_type == SymType::GlobalVar && v.name != DUMMY_VAR_NAME)
        .map(|v| v.name.clone())
        .collect();
    if global_names.len() > MAX_UDV_COUNT {
        return CompileResult::err(0, "", "Too many user-declared global variables");
    }
    for (idx, gname) in global_names.into_iter().enumerate() {
        udgv_lookup.insert(
            gname,
            USER_VAR_START_ADDRESS + (idx as u16) * USER_VAR_BYTE_WIDTH,
        );
    }

    // Combine root + functions and assign instruction addresses
    let mut final_assembly: Vec<Instruction> = Vec::new();
    let mut curr_addr = 0usize;
    for mut inst in ctx.root_assembly_list {
        inst.addr = curr_addr;
        curr_addr += inst.opcode.length();
        final_assembly.push(inst);
    }
    for fn_name in &ctx.func_assembly_order {
        if let Some(list) = ctx.func_assembly_dict.remove(fn_name) {
            for mut inst in list {
                inst.addr = curr_addr;
                curr_addr += inst.opcode.length();
                final_assembly.push(inst);
            }
        }
    }

    // Label -> address map
    let mut label_to_addr: HashMap<String, usize> = HashMap::new();
    for inst in &final_assembly {
        if let Some(ref lbl) = inst.label {
            if !lbl.is_empty() {
                label_to_addr.insert(lbl.clone(), inst.addr);
            }
        }
    }

    // Resolve symbolic payloads and build deduplicated string table in insertion order
    let mut string_order: Vec<Vec<u8>> = Vec::new();
    let mut string_addrs: HashMap<Vec<u8>, usize> = HashMap::new();

    for inst in &mut final_assembly {
        if inst.opcode == Op::Pushstr {
            if let Payload::Sym(ref raw_s) = inst.payload {
                let bytestr = replace_var_in_str(
                    raw_s,
                    inst.parent_func.as_deref(),
                    &ctx.reserved_vars,
                    &func_vars,
                    &udgv_lookup,
                );
                if !string_addrs.contains_key(&bytestr) {
                    string_addrs.insert(bytestr.clone(), 0);
                    string_order.push(bytestr.clone());
                }
                inst.payload = Payload::Bytes(bytestr);
            }
            continue;
        }
        if inst.opcode.length() == 1 {
            continue;
        }
        let Payload::Sym(ref sym) = inst.payload else {
            continue;
        };

        if let Some(&target_addr) = label_to_addr.get(sym) {
            inst.payload = Payload::Int(target_addr as i64);
        } else if inst.opcode == Op::Alloc {
            let locals_count = func_vars.get(sym).map_or(0, |(_, l)| l.len());
            inst.payload = Payload::Int(locals_count as i64);
        } else if matches!(inst.opcode, Op::Pushi | Op::Popi) {
            if let Some(&addr) = ctx.reserved_vars.get(sym.as_str()) {
                inst.payload = Payload::Int(addr as i64);
            } else if let Some(&addr) = udgv_lookup.get(sym) {
                inst.payload = Payload::Int(addr as i64);
            } else {
                return CompileResult::err(0, &inst.comment, format!("Unknown variable: {sym}"));
            }
        } else if matches!(inst.opcode, Op::Pushr | Op::Popr) {
            let Some(ref pf) = inst.parent_func else {
                return CompileResult::err(0, &inst.comment, format!("Unknown local: {sym}"));
            };
            let Some((args, locals)) = func_vars.get(pf) else {
                return CompileResult::err(0, &inst.comment, format!("Unknown function: {pf}"));
            };
            if inst.var_type == Some(SymType::FuncArg) {
                if let Some(pos) = args.iter().position(|a| a == sym) {
                    inst.payload = Payload::Int(((pos + 1) * 4) as i64);
                }
            } else if inst.var_type == Some(SymType::FuncLocalVar) {
                if let Some(pos) = locals.iter().position(|l| l == sym) {
                    inst.payload = Payload::Int(-(((pos + 1) * 4) as i64));
                }
            }
        }
    }

    let mut next_str_addr = curr_addr;
    for sbytes in &string_order {
        string_addrs.insert(sbytes.clone(), next_str_addr);
        next_str_addr += sbytes.len();
    }

    for inst in &mut final_assembly {
        if inst.opcode == Op::Pushstr {
            if let Payload::Bytes(ref b) = inst.payload {
                let addr = string_addrs[b];
                inst.opcode = Op::Pushc16;
                inst.payload = Payload::Int(addr as i64);
            }
        }
    }

    // Emit final binary bytes
    let mut bin_array = Vec::with_capacity(next_str_addr);
    for inst in &final_assembly {
        bin_array.push(inst.opcode.code());
        if let Payload::Int(val) = inst.payload {
            match inst.opcode {
                Op::Pushc8 => {
                    bin_array.push(val as u8);
                }
                Op::Pushc32 => {
                    bin_array.extend_from_slice(&(val as i32).to_le_bytes());
                }
                _ => {
                    if val >= 0 {
                        bin_array.extend_from_slice(&(val as u16).to_le_bytes());
                    } else {
                        bin_array.extend_from_slice(&(val as i16).to_le_bytes());
                    }
                }
            }
        }
    }
    for sbytes in &string_order {
        bin_array.extend_from_slice(sbytes);
    }

    if bin_array.len() > MAX_BIN_SIZE {
        return CompileResult::err(
            0,
            "",
            format!(
                "Bin size too large! This: {}, Max: {MAX_BIN_SIZE}",
                bin_array.len()
            ),
        );
    }

    CompileResult {
        is_success: true,
        error_comment: String::new(),
        error_line_number_starting_from_1: 0,
        error_line_str: String::new(),
        bin_array,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_empty_script_compiles_to_4_byte_halt() {
        let res = compile_duckyscript_to_dsb("", false, false);
        assert!(res.is_success);
        assert_eq!(res.bin_array, vec![0xFF, 0x02, 0x00, 0x0B]);
    }

    #[test]
    fn test_string_and_combo_script() {
        let res = compile_duckyscript_to_dsb("STRING Hello\nCTRL SHIFT z\n", false, false);
        assert!(res.is_success, "Err: {}", res.error_comment);
        assert!(res.bin_array.starts_with(&[0xFF, 0x02, 0x00]));
    }

    #[test]
    fn test_compile_utf8_string_cafe_au_lait() {
        let res = compile_duckyscript_to_dsb("STRING café au lait\n", false, false);
        assert!(res.is_success, "Err: {}", res.error_comment);
        assert!(res.bin_array.starts_with(&[0xFF, 0x02, 0x00]));
        assert!(
            res.bin_array
                .windows("café au lait".as_bytes().len())
                .any(|w| w == "café au lait".as_bytes()),
            "Expected UTF-8 bytes of 'café au lait' in compiled .dsb"
        );
    }

    #[test]
    fn test_sd_card_dsb_parity() {
        let mut candidate_roots: Vec<std::path::PathBuf> = Vec::new();
        let legacy_mount = Path::new("/run/media/millinr/DP24_2874");
        if legacy_mount.is_dir() {
            candidate_roots.push(legacy_mount.to_path_buf());
        }
        if let Some(mounted) = crate::hardware::storage::find_mounted_duckypad_sd() {
            if !candidate_roots.contains(&mounted) {
                candidate_roots.push(mounted);
            }
        }
        if let Some(home) = std::env::var_os("HOME") {
            let dev_profiles = std::path::PathBuf::from(home)
                .join(".local/share/duckydeck-studio/device_profiles");
            if let Ok(entries) = fs::read_dir(&dev_profiles) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_dir() {
                        candidate_roots.push(p);
                    }
                }
            }
        }

        let mut temp_fixture_root: Option<std::path::PathBuf> = None;
        if candidate_roots.is_empty() {
            let tmp_dir = std::env::temp_dir().join(format!(
                "duckydeck_sd_parity_fixture_{}",
                std::process::id()
            ));
            let prof_dir = tmp_dir.join("profile_Default");
            let _ = fs::create_dir_all(&prof_dir);
            let _ = fs::write(
                prof_dir.join("config.txt"),
                "z1 ESC\nz5 Tail\nx5 scale\nz6 Numpa\nz21 vol+\nBG_COLOR 255 255 255\n",
            );
            let fixtures: &[(&str, &str, &[u8])] = &[
                (
                    "key1",
                    "ESC",
                    &[255, 2, 0, 1, 41, 3, 65, 1, 41, 3, 66, 11],
                ),
                (
                    "key5",
                    "STRING .dingo-dory.ts.net\n",
                    &[
                        255, 2, 0, 1, 8, 0, 72, 11, 46, 100, 105, 110, 103, 111, 45, 100, 111, 114,
                        121, 46, 116, 115, 46, 110, 101, 116, 0,
                    ],
                ),
                (
                    "key6",
                    "GOTO_PROFILE Numpad\n",
                    &[255, 2, 0, 1, 8, 0, 84, 11, 78, 117, 109, 112, 97, 100, 0],
                ),
                (
                    "key21",
                    "MK_VOLUP\n",
                    &[255, 2, 0, 1, 64, 4, 65, 1, 64, 4, 66, 11],
                ),
                (
                    "key38",
                    "$_GV0=1\n\nIF $_GV0==$_GV1 && $_GV3==1 THEN\n  $_GV3=2\n  STRINGLN $_GV3\nELSE\n  IF $_GV3==2 || $_GV3==4 || $_GV3==6 || $_GV3==8 THEN\n    $_GV3=5\n  END_IF\n  STRINGLN PCCW_$_GV3\nEND_IF\n",
                    &[
                        255, 2, 0, 13, 4, 0, 252, 13, 2, 12, 252, 32, 2, 4, 252, 2, 0, 252, 32, 49,
                        6, 35, 0, 19, 2, 4, 12, 252, 1, 78, 0, 73, 7, 76, 0, 0, 19, 8, 2, 12, 252,
                        32, 19, 6, 2, 12, 252, 32, 19, 4, 2, 12, 252, 32, 19, 2, 2, 12, 252, 32,
                        50, 50, 50, 6, 71, 0, 19, 5, 4, 12, 252, 0, 1, 83, 0, 73, 0, 11, 31, 12,
                        252, 31, 0, 80, 67, 67, 87, 95, 31, 12, 252, 31, 0,
                    ],
                ),
                (
                    "key38-release",
                    "$_GV0=0\n\nIF $_GV0==$_GV1 && $_GV3==3 THEN\n  $_GV3=4\n  STRINGLN $_GV3\nELSE\n  IF $_GV3==2 || $_GV3==4 || $_GV3==6 || $_GV3==8 THEN\n    $_GV3=7\n  END_IF\n  STRINGLN RCCW_$_GV3\nEND_IF\n",
                    &[
                        255, 2, 0, 12, 4, 0, 252, 19, 3, 2, 12, 252, 32, 2, 4, 252, 2, 0, 252, 32,
                        49, 6, 36, 0, 19, 4, 4, 12, 252, 1, 79, 0, 73, 7, 77, 0, 0, 19, 8, 2, 12,
                        252, 32, 19, 6, 2, 12, 252, 32, 19, 4, 2, 12, 252, 32, 19, 2, 2, 12, 252,
                        32, 50, 50, 50, 6, 72, 0, 19, 7, 4, 12, 252, 0, 1, 84, 0, 73, 0, 11, 31,
                        12, 252, 31, 0, 82, 67, 67, 87, 95, 31, 12, 252, 31, 0,
                    ],
                ),
            ];
            for (stem, txt, dsb) in fixtures {
                let _ = fs::write(prof_dir.join(format!("{stem}.txt")), txt);
                let _ = fs::write(prof_dir.join(format!("{stem}.dsb")), dsb);
            }
            candidate_roots.push(tmp_dir.clone());
            temp_fixture_root = Some(tmp_dir);
        }

        let mut total = 0usize;
        let mut matched = 0usize;
        let mut mismatches = Vec::new();

        for sd_path in &candidate_roots {
            for entry in fs::read_dir(sd_path).into_iter().flatten().flatten() {
                let dir = entry.path();
                if !dir.is_dir() {
                    continue;
                }
                let Some(fname) = dir.file_name().and_then(|s| s.to_str()) else {
                    continue;
                };
                if !fname.starts_with("profile_") {
                    continue;
                }

                let mut ab_slots = HashSet::new();
                let mut dr_slots = HashSet::new();
                let cfg_path = dir.join("config.txt");
                if let Ok(cfg) = fs::read_to_string(&cfg_path) {
                    for line in cfg.lines() {
                        let t = line.trim();
                        if let Some(rest) = t.strip_prefix("ab ") {
                            if let Ok(idx) = rest.trim().parse::<usize>() {
                                ab_slots.insert(idx);
                            }
                        } else if let Some(rest) = t.strip_prefix("dr ") {
                            if let Ok(idx) = rest.trim().parse::<usize>() {
                                dr_slots.insert(idx);
                            }
                        }
                    }
                }

                for slot in 1..=58 {
                    for is_rel in [false, true] {
                        let txt_name = if is_rel {
                            format!("key{slot}-release.txt")
                        } else {
                            format!("key{slot}.txt")
                        };
                        let dsb_name = if is_rel {
                            format!("key{slot}-release.dsb")
                        } else {
                            format!("key{slot}.dsb")
                        };
                        let txt_path = dir.join(&txt_name);
                        let dsb_path = dir.join(&dsb_name);
                        if txt_path.exists() && dsb_path.exists() {
                            total += 1;
                            let script = fs::read_to_string(&txt_path).unwrap();
                            let expected = fs::read(&dsb_path).unwrap();
                            let res = compile_duckyscript_to_dsb(
                                &script,
                                ab_slots.contains(&slot),
                                dr_slots.contains(&slot),
                            );
                            if res.is_success && res.bin_array == expected {
                                matched += 1;
                            } else {
                                mismatches.push((
                                    format!("{}/{fname}/{txt_name}", sd_path.display()),
                                    res.is_success,
                                    res.error_comment,
                                    res.bin_array,
                                    expected,
                                ));
                            }
                        }
                    }
                }
            }
        }

        if let Some(tmp) = temp_fixture_root {
            let _ = fs::remove_dir_all(tmp);
        }

        println!("SD Card .dsb parity: {matched}/{total} matched");
        for (path, ok, err, got, exp) in &mismatches {
            println!(
                "MISMATCH {path}: ok={ok} err='{err}'\n  got ({}) = {:02x?}\n  exp ({}) = {:02x?}",
                got.len(),
                &got[..got.len().min(48)],
                exp.len(),
                &exp[..exp.len().min(48)]
            );
        }
        assert!(
            total > 0,
            "SD card .dsb parity test did not check any files (total == 0)"
        );
        assert!(
            mismatches.is_empty(),
            "{} out of {total} .dsb files differed",
            mismatches.len()
        );
    }
}

