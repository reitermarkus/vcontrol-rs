#[cfg(feature = "webthing")]
use {
  std::sync::{Arc, RwLock},
  webthing::Thing
};

use crate::Command;
use crate::types::{DeviceIdent, DeviceIdentF0};
use crate::{Error, Optolink, Device, device::DEVICES, Protocol, Value, OutputValue};

#[cfg(feature = "webthing")]
mod thing;

/// Representation of an `Optolink` connection to a specific `Device` using a specific `Protocol`.
#[derive(Debug)]
pub struct VControl {
  optolink: Optolink,
  device: &'static Device,
  connected: bool,
  protocol: Protocol,
}

impl VControl {
  fn renegotiate(&mut self) -> Result<(), Error> {
    if !self.connected {
      self.protocol.negotiate(&mut self.optolink)?;
      self.connected = true;
    }

    Ok(())
  }

  fn detect_device(device_ident: DeviceIdent, device_ident_f0: Option<DeviceIdentF0>) -> Option<&'static Device> {
    let devices = DEVICES.entries().filter(|(device_ident_range, _)| device_ident.id == device_ident_range.id);

    if let Some(device_ident_f0) = device_ident_f0 {
      if (192..=203).contains(&device_ident.id) && device_ident.software_index >= 200 {
        // Find exact F0 match.
        for (device_ident_range, device) in devices.clone() {
          if let Some(f0) = device_ident_range.f0 {
            if device_ident_f0.0 == f0 {
              return Some(device)
            }
          }
        }

        // Find match in F0 range.
        for (device_ident_range, device) in devices.clone() {
          if let Some((f0, f0_till)) = device_ident_range.f0.zip(device_ident_range.f0_till) {
            let f0_range = f0..=f0_till;

            if f0_range.contains(&device_ident_f0.0) {
              return Some(device)
            }
          }
        }
      }
    }

    let mut device_fallback = None;

    // Find exact hardware/software index match.
    for (device_ident_range, device) in devices.clone() {
      if let Some((hardware_index, software_index)) = device_ident_range.hardware_index.zip(device_ident_range.software_index) {
        if device_ident.hardware_index == hardware_index && device_ident.software_index == software_index {
          return Some(device)
        }
      }

      device_fallback = Some(*device);
    }

    // Find match in hardware/software index range.
    for (device_ident_range, device) in devices.clone() {
      if let Some((hardware_index, software_index)) = device_ident_range.hardware_index.zip(device_ident_range.software_index) {
        if let Some((hardware_index_till, software_index_till)) = device_ident_range.hardware_index.zip(device_ident_range.software_index) {
          let hardware_index_range = hardware_index..=hardware_index_till;
          let software_index_range = software_index..=software_index_till;

          if hardware_index_range.contains(&device_ident.hardware_index) && software_index_range.contains(&device_ident.software_index) {
            return Some(device)
          }
        }
      }
    }

    device_fallback
  }

  /// Automatically detect the `Device` and `Protocol` and connect to it.
  pub fn connect(mut optolink: Optolink) -> Result<Self, Error> {
    let (connected, protocol) = if let Some(protocol) = Protocol::detect(&mut optolink) {
      log::debug!("Protocol detected: {:?}", protocol);
      (true, protocol)
    } else {
      let protocol = Protocol::Vs1;
      log::warn!("No protocol detected, defaulting to {:?}.", protocol);
      (false, protocol)
    };

    let mut buf = [0; 8];
    protocol.get(&mut optolink, 0x00f8, &mut buf)?;
    let device_ident = DeviceIdent::from_bytes(&buf);

    let mut buf = [0; 2];
    let device_ident_f0 = match protocol.get(&mut optolink, 0x00f0, &mut buf) {
      Ok(()) => Some(DeviceIdentF0::from_bytes(&buf)),
      Err(_) => None, // TODO: Use specific error type.
    };

    let mut device = Self::detect_device(device_ident, device_ident_f0);

    let device = if let Some(device) = device {
      log::debug!("Device detected: {}", device.name());
      device
    } else {
      return Err(Error::UnsupportedDevice(device_ident, device_ident_f0))
    };

    let mut vcontrol = VControl { optolink, device, connected, protocol };
    vcontrol.renegotiate()?;
    Ok(vcontrol)
  }

  pub fn device(&self) -> &'static Device {
    self.device
  }

  pub fn protocol(&mut self) -> Protocol {
    self.protocol
  }

  fn command_by_name(&self, command: &str) -> Result<&'static Command, Error> {
    if let Some(system_command) = crate::commands::system_command(command) {
      Ok(system_command)
    } else if let Some(device_command) = self.device.command(command) {
      Ok(device_command)
    } else {
      Err(Error::UnsupportedCommand(command.to_owned()))
    }
  }

  /// Gets the value for the given command.
  ///
  /// If the command specified is not available, an IO error of the kind `AddrNotAvailable` is returned.
  pub fn get(&mut self, command: &str) -> Result<OutputValue, Error> {
    self.renegotiate()?;

    let command = self.command_by_name(command)?;
    match command.get(&mut self.optolink, self.protocol) {
      Ok(value) => {
        let mapping = if let Value::Error(ref error) = value {
          Some(self.device.errors())
        } else if let Some(ref mapping) = command.mapping {
          Some(mapping)
        } else {
          None
        };

        if let (Value::Int(value), Some(mapping)) = (&value, command.mapping.as_ref()) {
          let n = *value as i32;
          if !mapping.contains_key(&n) {
            return Err(Error::UnknownEnumVariant(format!("No enum mapping found for {}.", n)))
          }
        }

        Ok(OutputValue {
          value,
          unit: command.unit,
          mapping,
        })
      },
      Err(err) => {
        self.connected = false;
        Err(err)
      }
    }
  }

  /// Sets the value for the given command.
  ///
  /// If the command specified is not available, an IO error of the kind `AddrNotAvailable` is returned.
  pub fn set(&mut self, command: &str, input: Value) -> Result<(), Error> {
    self.renegotiate()?;

    let command = self.command_by_name(command)?;
    let res = command.set(&mut self.optolink, self.protocol, input);
    if res.is_err() {
      self.connected = false;
    }
    res
  }

  #[cfg(feature = "webthing")]
  pub fn into_thing(self) -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    self::thing::make_thing(self)
  }
}
