#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Keycode {
    // Special
    No = 0x00,
    Trans = 0x01,           // _______ in QMK
    MomentaryLayer1 = 0xF1, // Custom code for MO(_FL)
    MomentaryLayer2 = 0xF2, // Custom code for MO(_CL)

    // Standard Keys
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,

    Num1 = 0x1E,
    Num2 = 0x1F,
    Num3 = 0x20,
    Num4 = 0x21,
    Num5 = 0x22,
    Num6 = 0x23,
    Num7 = 0x24,
    Num8 = 0x25,
    Num9 = 0x26,
    Num0 = 0x27,

    Enter = 0x28,
    Escape = 0x29,
    BSpace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Equal = 0x2E,
    LBracket = 0x2F,
    RBracket = 0x30,
    BSlash = 0x31,
    Semicolon = 0x33,
    Quote = 0x34,
    Grave = 0x35,
    Comma = 0x36,
    Dot = 0x37,
    Slash = 0x38,
    Caps = 0x39,

    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,

    PScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PgUp = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    PgDown = 0x4E,

    Right = 0x4F,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,

    LCtrl = 0xE0,
    LShift = 0xE1,
    LAlt = 0xE2,
    LGui = 0xE3,
    RCtrl = 0xE4,
    RShift = 0xE5,
    RAlt = 0xE6,
    RGui = 0xE7,

    // Custom function keys
    GraveEsc = 0xF0, // QK_GESC
    Reset = 0xF3,    // QK_BOOT
}
