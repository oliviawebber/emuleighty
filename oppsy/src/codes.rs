use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum MainInstructions {
    #[deku(id = "0x00")]
    NOP,
    #[deku(id = "0x01")]
    LDBC(u16),
    #[deku(id = "0x03")]
    INCBC,
    #[deku(id = "0x21")]
    LDHL(u16),
    #[deku(id = "0x23")]
    INCHL,
    #[deku(id = "0x3C")]
    INCA,
    #[deku(id = "0x3E")]
    LDA(u8),
    #[deku(id = "0x76")]
    HALT,
    #[deku(id = "0x77")]
    LDHLA,
    #[deku(id = "0x7E")]
    LDAHL,
    #[deku(id = "0xAF")]
    XORA,
    #[deku(id = "0xC3")]
    JPN(u16),
    #[deku(id = "0xC8")]
    RETZ,
    #[deku(id = "0xCD")]
    CALL(u16),
    #[deku(id = "0xD3")]
    OUT(u8),
    #[deku(id = "0xE1")]
    POPHL,
    #[deku(id = "0xE5")]
    PUSHHL,
    #[deku(id = "0xFE")]
    CPN(u8),
}
