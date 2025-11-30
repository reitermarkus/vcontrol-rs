use crate::{Command, Device, Error, Optolink, OutputValue, Protocol, Value};

/// Representation of an `Optolink` connection to a specific `Device` using a specific `Protocol`.
#[derive(Debug)]
pub struct VControl {
  optolink: Optolink,
  device: &'static Device,
  connected: bool,
  protocol: Protocol,
}

impl VControl {
  async fn renegotiate(&mut self) -> Result<(), Error> {
    log::trace!("VControl::reneogiate()");

    if self.connected {
      log::trace!("VControl::reneogiate(): already connected");
      return Ok(());
    }

    let mut reinitialized = false;
    loop {
      match self.protocol.negotiate(&mut self.optolink).await {
        Ok(()) => {
          self.connected = true;
          return Ok(());
        },
        Err(err) if reinitialized => return Err(err.into()),
        Err(err) => {
          match self.optolink.reinitialize().await {
            Ok(()) => {
              log::info!("Optolink port successfully re-initialized after error.");
              reinitialized = true;
              continue;
            },
            Err(err) => {
              log::warn!("Failed to re-initialize Optolink port after error: {err}");
            },
          }

          return Err(err.into());
        },
      }
    }
  }

  /// Automatically detect the `Device` and `Protocol` and connect to it.
  pub async fn connect(mut optolink: Optolink) -> Result<Self, Error> {
    log::trace!("VControl::connect(…)");

    let (connected, protocol) = if let Some(protocol) = Protocol::detect(&mut optolink).await {
      log::debug!("Protocol detected: {protocol}");
      (true, protocol)
    } else {
      let protocol = Protocol::Vs1;
      log::warn!("No protocol detected, defaulting to {protocol}.");
      (false, protocol)
    };

    let (device_id, device_id_f0) = match crate::commands::system::DEVICE_ID.get(&mut optolink, protocol).await? {
      Value::DeviceId(device_id) => {
        let device_id_f0 = match crate::commands::system::DEVICE_ID_F0.get(&mut optolink, protocol).await {
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
          vcontrol.renegotiate().await?;
          return Ok(vcontrol);
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
  pub async fn get(&mut self, command: &str) -> Result<OutputValue, Error> {
    log::trace!("VControl::get({command:?})");

    let command = self.command_by_name(command)?;

    self.renegotiate().await?;
    match command.get(&mut self.optolink, self.protocol).await {
      Ok(value) => {
        let mapping = if let Value::Error(ref _error) = value {
          Some(self.device.errors())
        } else if let Some(ref mapping) = command.mapping {
          Some(mapping)
        } else {
          None
        };

        Ok(OutputValue { value, unit: command.unit, mapping })
      },
      Err(err) => {
        self.connected = false;
        Err(err)
      },
    }
  }

  /// Sets the value for the given command.
  pub async fn set(&mut self, command: &str, input: Value) -> Result<(), Error> {
    log::trace!("VControl::set({command:?}, {input:?})");

    let command = self.command_by_name(command)?;

    self.renegotiate().await?;
    match command.set(&mut self.optolink, self.protocol, input).await {
      Ok(()) => Ok(()),
      Err(err) => {
        self.connected = false;
        Err(err)
      },
    }
  }
}
