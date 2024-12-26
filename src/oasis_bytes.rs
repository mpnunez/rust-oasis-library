pub struct OasisBytes {}

impl OasisBytes {
    pub const MAGIC_BYTES: &'static str = "%SEMI-OASIS\r\n";
    pub const CURVI_MAGIC_BYTES: &'static str = "%SEMI-OASIS-CURVILINEAR\r\n";
    pub const VERSION_STRING: &'static str = "1.0";
}
