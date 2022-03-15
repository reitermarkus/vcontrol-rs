#[cfg(feature = "webthing")]
use {
  std::sync::{Arc, RwLock},
  webthing::Thing
};

use crate::Command;
use crate::device::{DeviceId, DeviceIdF0};
use crate::{Error, Optolink, Device, Protocol, Value, OutputValue};

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
    let device_ident = DeviceId::from_bytes(&buf);

    let mut buf = [0; 2];
    let device_ident_f0 = match protocol.get(&mut optolink, 0x00f0, &mut buf) {
      Ok(()) => Some(DeviceIdF0::from_bytes(&buf)),
      Err(_) => None, // TODO: Use specific error type.
    };

    let mut device = Device::detect(device_ident, device_ident_f0);

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
