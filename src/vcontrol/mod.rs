use crate::Command;
use crate::{Error, Optolink, Device, Protocol, Value, OutputValue};

#[cfg(feature = "webthing")]
pub mod thing;

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

    let (device_id, device_id_f0) = match crate::commands::system::DEVICE_ID.get(&mut optolink, protocol)? {
      Value::DeviceId(device_id) => {
        let device_id_f0 = match crate::commands::system::DEVICE_ID_F0.get(&mut optolink, protocol) {
          Ok(Value::DeviceIdF0(device_id_f0)) => Some(device_id_f0),
          Ok(Value::Empty) => None,
          Ok(value) => unreachable!("expected DeviceIdF0, got {:?}", value),
          // TODO: Check for more specific error type.
          Err(err) => {
            log::debug!("Failed to get `device_id_f0`: {}", err);
            None
          },
        };

        if let Some(device) = Device::detect(device_id, device_id_f0) {
          log::debug!("Device detected: {}", device.name());

          let mut vcontrol = VControl { optolink, device, connected, protocol };
          vcontrol.renegotiate()?;
          return Ok(vcontrol)
        }

        (device_id, device_id_f0)
      },
      value => unreachable!("expected DeviceId, got {:?}", value),
    };

    Err(Error::UnsupportedDevice(device_id, device_id_f0))
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
        let mapping = if let Value::Error(ref _error) = value {
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
}
