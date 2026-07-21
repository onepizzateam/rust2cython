pub enum Msg { Text(*const u8), Count(u32), Empty }
pub extern "C" fn msg_tag(m: u32) -> u32 { m }
