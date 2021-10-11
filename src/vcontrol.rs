use crate::{Error, Optolink, Device, Protocol, Value};

#[derive(Debug)]
pub struct VControl<D: Device> {
  device: Optolink,
  phantom: std::marker::PhantomData<D>,
  connected: bool,
}

impl<D: Device> VControl<D> {
  fn renegotiate(&mut self) -> Result<(), Error> {
    if !self.connected {
      D::Protocol::negotiate(&mut self.device)?;
    }

    Ok(())
  }

  pub fn connect(device: Optolink) -> Result<VControl<D>, Error> {
    let mut vcontrol = VControl { device, phantom: std::marker::PhantomData, connected: false };
    vcontrol.renegotiate()?;
    Ok(vcontrol)
  }

  /// Gets the value for the given command.
  ///
  /// If the command specified is not available, an IO error of the kind `AddrNotAvailable` is returned.
  pub fn get(&mut self, command: &str) -> Result<Value, Error> {
    self.renegotiate()?;

    if let Some(command) = D::command(command) {
      let res = command.get::<D::Protocol>(&mut self.device);
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

    if let Some(command) = D::command(command) {
      let res = command.set::<D::Protocol>(&mut self.device, input);
      if res.is_err() {
        self.connected = false;
      }
      res
    } else {
      Err(Error::UnsupportedCommand(command.to_owned()))
    }
  }
}
