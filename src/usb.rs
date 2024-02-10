// /// Impl from [Class Codes](https://usb.org/defined-class-codes)
// ///
// #[repr(u8)]
// pub enum ClassCode {
//   Device = 0x00,
//   Audio = 0x01,
//   Communication = 0x02,
//   HID = 0x03,
//   Physical = 0x05,
//   Image = 0x06,
//   Printer = 0x07,
//   MassStorage = 0x08,
//   Hub = 0x09,
//   CDCData = 0x0A,
//   SmartCard = 0x0B,
//   ContentSecurity = 0x0D,
//   Video = 0x0E,
//   PersonalHealthcare = 0x0F,
//   AudioVideoDevice = 0x10,
//   BillboardDevice = 0x11,
//   USBTypeCBridge = 0x12,
//   USBBulkDisplayProtocol = 0x13,
//   MCTPOverUSBProtocol = 0x14,
//   I3CDevice = 0x3C,
//   DiagnosticDevice = 0xDC,
//   WirelessController = 0xE0,
//   Miscellaneous = 0xEF,
//   ApplicationSpecific {
//     base: u8,
//     subclass: u8,
//     protocol: u8,
//   },
//   VendorSpecific = 0xFF,
// }

pub struct Class {
  pub base: u8,
  pub subclass: u8,
  pub protocol: u8,
}

impl Default for Class {
  fn default() -> Self {
    Self {
      base: 0x00,
      subclass: 0x00,
      protocol: 0x00,
    }
  }
}
