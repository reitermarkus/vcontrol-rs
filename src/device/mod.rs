use phf;

use crate::{Command};
use crate::types::{DeviceId, DeviceIdF0};

mod device_id_range;
pub use device_id_range::DeviceIdRange;

const USE_HARDWARE_INDEX: bool = false;

#[allow(clippy::unreadable_literal)]
mod codegen {
  use super::*;

  include!(concat!(env!("OUT_DIR"), "/devices.rs"));
}

pub use self::codegen::*;

/// Representation of a heating system device.
#[derive(Debug)]
pub struct Device {
  name: &'static str,
  commands: &'static phf::Map<&'static str, &'static Command>,
  errors: &'static phf::Map<i32, &'static str>,
}

impl Device {
  /// Get the name of the device.
  pub fn name(&self) -> &'static str {
    self.name
  }

  /// Get all supported commands for the device.
  pub fn commands(&self) -> &'static phf::Map<&'static str, &'static Command> {
    self.commands
  }

  /// Get a specific command for the device, if it is supported.
  pub fn command(&self, name: impl AsRef<str>) -> Option<&'static Command> {
    self.commands.get(name.as_ref()).copied()
  }

  /// Get mapping from error codes to strings.
  pub fn errors(&self) -> &'static phf::Map<i32, &'static str> {
    self.errors
  }

  /// Detect a device by identifier.
  pub fn detect(device_id: DeviceId, device_id_f0: Option<DeviceIdF0>) -> Option<&'static Self> {
    let devices = DEVICES.entries().filter(|(device_id_range, _)| device_id.id == device_id_range.id);

    if let Some(device_id_f0) = device_id_f0 {
      if (192..=203).contains(&device_id.id) && device_id.software_index >= 200 {
        // Find exact F0 match.
        for (device_id_range, device) in devices.clone() {
          if let Some(f0) = device_id_range.f0 {
            if device_id_f0.0 == f0 {
              log::debug!("Found device with exact ID and F0.");

              return Some(device)
            }
          }
        }

        // Find match in F0 range.
        for (device_id_range, device) in devices.clone() {
          if let Some((f0, f0_till)) = device_id_range.f0.zip(device_id_range.f0_till) {
            let f0_range = f0..=f0_till;

            if f0_range.contains(&device_id_f0.0) {
              log::debug!("Found device with exact ID and F0 in range {:?}.", f0_range);

              return Some(device)
            }
          }
        }
      }
    }

    let mut device_fallback = None;

    // Find exact hardware/software index match.
    for (device_id_range, device) in devices.clone() {
      if let Some((hardware_index, software_index)) = device_id_range.hardware_index.zip(device_id_range.software_index) {
        if (!USE_HARDWARE_INDEX || device_id.hardware_index == hardware_index) && device_id.software_index == software_index {
          log::debug!("Found device with exact ID, hardware index and software index.");

          return Some(device)
        }
      }

      device_fallback = Some(*device);
    }

    // Find match in hardware/software index range.
    for (device_id_range, device) in devices.clone() {
      if let Some((hardware_index, software_index)) = device_id_range.hardware_index.zip(device_id_range.software_index) {
        if let Some((hardware_index_till, software_index_till)) = device_id_range.hardware_index_till.zip(device_id_range.software_index_till) {
          let hardware_index_range = hardware_index..=hardware_index_till;
          let software_index_range = software_index..=software_index_till;

          if (!USE_HARDWARE_INDEX || hardware_index_range.contains(&device_id.hardware_index)) && software_index_range.contains(&device_id.software_index) {
            log::debug!(
              "Found device with exact ID, hardware index in range {:?} and software index in range {:?}.",
              hardware_index_range, software_index_range,
            );
            return Some(device)
          }
        }
      }
    }

    if let Some(device) = device_fallback {
      log::debug!("Found device with exact ID.");
      return Some(device)
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn detect_vscot_ho1_4() {
    let device_id = DeviceId::from_bytes(&[0x20, 0xCB, 0x00, 0x08, 0x00, 0x00, 0x01, 0x46]);
    let device = Device::detect(device_id, None).unwrap();
    assert_eq!(device.name(), "VScotHO1_4");
  }

  #[test]
  fn detect_vscot_ho1_72() {
    let device_id = DeviceId::from_bytes(&[0x20, 0xCB, 0x03, 0x51, 0x00, 0x00, 0x01, 0x46]);
    let device = Device::detect(device_id, None).unwrap();
    assert_eq!(device.name(), "VScotHO1_72");
  }

  #[test]
  fn detect_ecotronic() {
    let device_id = DeviceId::from_bytes(&[0x20, 0x34, 0x00, 0x18, 0x00, 0x00, 0x0f, 0x0f]);
    let device = Device::detect(device_id, None).unwrap();
    assert_eq!(device.name(), "Ecotronic");
  }
}
