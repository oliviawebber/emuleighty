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
}
