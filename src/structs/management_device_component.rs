use super::*;

/// # Management Device Component (Type 35)
///
/// This structure associates a cooling device or environmental probe with structures that define the
/// controlling hardware device and (optionally) the component’s thresholds.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosManagementDeviceComponent<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosManagementDeviceComponent<'a> {
    const STRUCT_TYPE: u8 = 35u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosManagementDeviceComponent<'a> {
    /// Number of the string that contains additional descriptive information about the component
    pub fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Handle, or instance number, of the Management Device that contains this component
    pub fn management_device_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x05)
    }

    /// Handle, or instance number, of the probe or cooling device that defines this component
    pub fn component_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x07)
    }

    /// Handle, or instance number, associated with the device
    /// thresholds;
    /// A value of 0FFFFh indicates that no Threshold Data
    /// structure is associated with this component.
    pub fn threshold_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x09)
    }
}

impl fmt::Debug for SMBiosManagementDeviceComponent<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosManagementDeviceComponent>())
            .field("header", &self.parts.header)
            .field("description", &self.description())
            .field("management_device_handle", &self.management_device_handle())
            .field("component_handle", &self.component_handle())
            .field("threshold_handle", &self.threshold_handle())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type35 = vec![
            0x23, 0x0B, 0x29, 0x00, 0x01, 0x26, 0x00, 0x27, 0x00, 0x28, 0x00, 0x44, 0x65, 0x66,
            0x61, 0x75, 0x6C, 0x74, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type35.as_slice());
        let test_struct = SMBiosManagementDeviceComponent::new(&parts);

        assert_eq!(
            test_struct.description(),
            Some("Default string".to_string())
        );
        // assert_eq!(test_struct.management_device_handle(), Some(Handle(38)));
        // assert_eq!(test_struct.component_handle(), Some(Handle(39)));
        // assert_eq!(test_struct.threshold_handle(), Some(Handle(40)));
    }
}