use super::*;

pub struct SMBiosUnknown<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosUnknown<'a> {
    pub fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        SMBiosUnknown { parts: parts }
    }

    pub fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl fmt::Debug for SMBiosUnknown<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosUnknown>())
            .field("header", &self.parts.header)
            .field("strings", &self.parts.strings)
            .finish()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_unknown_oem_type() {
        // For testing we've borrowed a language information type (0x0D) structure and change its type to 0x99 (> 0x7F are OEM types)
        let unknown_bytes = vec![
            0x99u8, 0x16, 0x21, 0x00, // unknown data
            0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, // "en|US|iso8859-1"
            0x65, 0x6E, 0x7C, 0x55, 0x53, 0x7C, 0x69, 0x73, 0x6F, 0x38, 0x38, 0x35, 0x39, 0x2D,
            0x31, 0x00, // "fr|FR|iso8859-1"
            0x66, 0x72, 0x7C, 0x46, 0x52, 0x7C, 0x69, 0x73, 0x6F, 0x38, 0x38, 0x35, 0x39, 0x2D,
            0x31, 0x00, // "ja|JP|unicode"
            0x6A, 0x61, 0x7C, 0x4A, 0x50, 0x7C, 0x75, 0x6E, 0x69, 0x63, 0x6F, 0x64, 0x65, 0x00,
            // end of structure
            0x00,
        ];

        let parts = SMBiosStructParts::new(unknown_bytes.as_slice());
        let unknown = SMBiosUnknown::new(&parts);

        // header tests
        assert_eq!(*unknown.parts().header.handle(), 0x0021);
        assert_eq!(unknown.parts().header.length(), 0x16);

        // basic field tests
        //assert_eq!(bios_language_information.current_language().expect("current_language field exists"), "en|US|iso8859-1".to_string());

        // debug print test
        println!("unknown structure: {:?}", unknown);
    }
}
