use crate::{Error, Optolink, Device, device::DEVICES, Protocol, Value};

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
      let protocol = Protocol::Kw2;
      log::warn!("No protocol detected, defaulting to {:?}.", protocol);
      (false, protocol)
    };

    let mut buf = [0; 2];
    protocol.get(&mut optolink, 0x00f8, &mut buf)?;
    let device_id = u16::from_be_bytes(buf);
    let device_id_full = ((device_id as u64) << 32) + (0 << 16) + (0 << 0);

    let device = if let Some(device) = DEVICES.get(&device_id_full) {
      log::debug!("Device detected: {}", device.name());
      *device
    } else {
      return Err(Error::UnsupportedDevice(device_id))
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

  /// Gets the value for the given command.
  ///
  /// If the command specified is not available, an IO error of the kind `AddrNotAvailable` is returned.
  pub fn get(&mut self, command: &str) -> Result<Value, Error> {
    self.renegotiate()?;

    if let Some(command) = self.device.command(command) {
      let res = command.get(&mut self.optolink, self.protocol);
      if res.is_err() {
        self.connected = false
      }
      res
    } else {
      Err(Error::UnsupportedCommand(command.to_owned()))
    }
  }

  /// Sets the value for the given command.
  ///
  /// If the command specified is not available, an IO error of the kind `AddrNotAvailable` is returned.
  pub fn set(&mut self, command: &str, input: &Value) -> Result<(), Error> {
    self.renegotiate()?;

    if let Some(command) = self.device.command(command) {
      let res = command.set(&mut self.optolink, self.protocol, input);
      if res.is_err() {
        self.connected = false;
      }
      res
    } else {
      Err(Error::UnsupportedCommand(command.to_owned()))
    }
  }
}
