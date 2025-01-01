pub struct OasisBytes {}

impl OasisBytes {
    pub const MAGIC_BYTES: &'static str = "%SEMI-OASIS\r\n";
    pub const CURVI_MAGIC_BYTES: &'static str = "%SEMI-OASIS-CURVILINEAR\r\n";
    pub const VERSION_STRING: &'static str = "1.0";

    pub const TABLE_OFFSETS_IN_START_RECORD: u8 = 0;
    pub const TABLE_OFFSETS_IN_END_RECORD: u8 = 1;

    pub const END_RECORD_VALIDATION_NONE: u8 = 0;
    pub const END_RECORD_VALIDATION_CRC32: u8 = 1;
    pub const END_RECORD_VALIDATION_CHECKSUM32: u8 = 2;
}
