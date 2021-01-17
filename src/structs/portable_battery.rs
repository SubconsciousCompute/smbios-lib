use super::*;

/// # Portable Battery (Type 22)
///
/// This structure describes the attributes of the portable battery or batteries for the system. The structure
/// contains the static attributes for the group. Each structure describes a single battery pack’s attributes.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosPortableBattery<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosPortableBattery<'a> {
    const STRUCT_TYPE: u8 = 22u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosPortableBattery<'a> {
    /// Identifies the location of the battery
    pub fn location(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Names the company that manufactured the battery
    pub fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x05)
    }

    /// The date on which the battery was manufactured.
    ///
    /// Version 2.2+ implementations that use a Smart
    /// Battery set this field to 0 (no string) to indicate
    /// that the SBDS Manufacture Date field contains
    /// the information.
    pub fn manufacture_date(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// The serial number for the battery
    ///
    /// Version 2.2+ implementations that use a Smart
    /// Battery set this field to 0 (no string) to indicate
    /// that the SBDS Serial Number field contains the
    /// information.
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    /// Names the battery device
    ///
    /// EXAMPLE: "DR-36"
    pub fn device_name(&self) -> Option<String> {
        self.parts.get_field_string(0x08)
    }

    /// Identifies the battery chemistry
    ///
    /// Version 2.2+ implementations that use a Smart
    /// Battery set this field to 02h (Unknown) to
    /// indicate that the SBDS Device Chemistry field
    /// contains the information.
    pub fn device_chemistry(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    /// Design capacity of the battery in mWatt-hours
    ///
    /// If the value is unknown, the field contains 0.
    ///
    /// For version 2.2+ implementations, this value is
    /// multiplied by the Design Capacity Multiplier to
    /// produce the actual value.
    pub fn design_capacity(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    /// Design voltage of the battery in mVolts
    ///
    /// If the value is unknown, the field contains 0.
    pub fn design_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    /// Contains the Smart Battery Data Specification version number
    /// supported by this battery
    ///
    /// If the battery does not support the function, no
    /// string is supplied.
    pub fn sbds_version_number(&self) -> Option<String> {
        self.parts.get_field_string(0x0E)
    }

    /// Maximum error (as a percentage in the range 0
    /// to 100) in the Watt-hour data reported by the
    /// battery, indicating an upper bound on how much
    /// additional energy the battery might have above
    /// the energy it reports having
    ///
    /// If the value is unknown, the field contains FFh.
    pub fn maximum_error_in_battery_data(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }

    /// 16-bit value that identifies the battery’s serial
    /// number
    ///
    /// This value, when combined with the
    /// Manufacturer, Device Name, and Manufacture
    /// Date, uniquely identifies the battery. The Serial
    /// Number field must be set to 0 (no string) for this
    /// field to be valid.
    pub fn sbds_serial_number(&self) -> Option<u16> {
        self.parts.get_field_word(0x10)
    }

    /// Date the cell pack was manufactured, in packed
    /// format
    pub fn sbds_manufacture_date(&self) -> Option<u16> {
        self.parts.get_field_word(0x12)
    }

    /// Number of the string that identifies the battery
    /// chemistry (for example, “PbAc”)
    /// The Device Chemistry field must be set to 02h
    /// (Unknown) for this field to be valid.
    pub fn sbds_device_chemistry(&self) -> Option<String> {
        self.parts.get_field_string(0x14)
    }

    /// Multiplication factor of the Design Capacity
    /// value, which assures that the mWatt hours value
    /// does not overflow for SBDS implementations
    ///
    /// The multiplier default is 1, SBDS
    /// implementations use the value 10 to correspond
    /// to the data as returned from the SBDS Function
    /// 18h.
    pub fn design_capacity_multiplier(&self) -> Option<u8> {
        self.parts.get_field_byte(0x15)
    }

    /// Contains OEM- or BIOS vendor-specific
    /// information
    pub fn oem_specific(&self) -> Option<u32> {
        self.parts.get_field_dword(0x16)
    }
}

impl fmt::Debug for SMBiosPortableBattery<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPortableBattery>())
            .field("header", &self.parts.header)
            .field("location", &self.location())
            .field("manufacturer", &self.manufacturer())
            .field("manufacture_date", &self.manufacture_date())
            .field("serial_number", &self.serial_number())
            .field("device_name", &self.device_name())
            .field("device_chemistry", &self.device_chemistry())
            .field("design_capacity", &self.design_capacity())
            .field("design_voltage", &self.design_voltage())
            .field("sbds_version_number", &self.sbds_version_number())
            .field(
                "maximum_error_in_battery_data",
                &self.maximum_error_in_battery_data(),
            )
            .field("sbds_serial_number", &self.sbds_serial_number())
            .field("sbds_manufacture_date", &self.sbds_manufacture_date())
            .field("sbds_device_chemistry", &self.sbds_device_chemistry())
            .field(
                "design_capacity_multiplier",
                &self.design_capacity_multiplier(),
            )
            .field("oem_specific", &self.oem_specific())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type22 = vec![
            0x16, 0x1A, 0x2E, 0x00, 0x01, 0x02, 0x00, 0x00, 0x03, 0x02, 0xFB, 0x11, 0xD0, 0x39,
            0x04, 0xFF, 0xC7, 0x02, 0x7A, 0x42, 0x05, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x52, 0x65,
            0x61, 0x72, 0x00, 0x53, 0x4D, 0x50, 0x00, 0x34, 0x35, 0x4E, 0x31, 0x30, 0x37, 0x31,
            0x00, 0x30, 0x33, 0x2E, 0x30, 0x31, 0x00, 0x4C, 0x69, 0x50, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type22.as_slice());
        let test_struct = SMBiosPortableBattery::new(&parts);

        assert_eq!(test_struct.location(), Some("Rear".to_string()));
        assert_eq!(test_struct.manufacturer(), Some("SMP".to_string()));
        assert_eq!(test_struct.manufacture_date(), None);
        assert_eq!(test_struct.serial_number(), None);
        assert_eq!(test_struct.device_name(), Some("45N1071".to_string()));
        assert_eq!(test_struct.device_chemistry(), Some(2));
        assert_eq!(test_struct.design_capacity(), Some(4603));
        assert_eq!(test_struct.design_voltage(), Some(14800));
        assert_eq!(test_struct.sbds_version_number(), Some("03.01".to_string()));
        assert_eq!(test_struct.maximum_error_in_battery_data(), Some(255));
        assert_eq!(test_struct.sbds_serial_number(), Some(711));
        assert_eq!(test_struct.sbds_manufacture_date(), Some(17018));
        assert_eq!(test_struct.sbds_device_chemistry(), Some("LiP".to_string()));
        assert_eq!(test_struct.design_capacity_multiplier(), Some(10));
        assert_eq!(test_struct.oem_specific(), Some(0));
    }
}