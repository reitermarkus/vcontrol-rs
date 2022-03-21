use core::pin::{Pin};
use core::task::{Context, Poll};
use std::fmt;
use std::io;
use std::net::{ToSocketAddrs, SocketAddr};
use std::time::Duration;

use pin_project::pin_project;
use tokio::{io::{ReadBuf, AsyncRead, AsyncWrite}, net::TcpStream};
use tokio_serial::{SerialStream, SerialPortBuilderExt, DataBits, FlowControl, StopBits, Parity};

#[pin_project(project = DeviceProj)]
enum Device {
  Tty(#[pin] SerialStream),
  Stream(#[pin] TcpStream),
}

impl fmt::Debug for Device   {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Device::Tty(tty) => tty.fmt(f),
      Device::Stream(stream) => stream.fmt(f),
    }
  }
}

/// An Optolink connection via either a serial or TCP connection.
#[derive(Debug)]
#[pin_project]
pub struct Optolink {
  #[pin]
  device: Device,
  timeout: Option<Duration>,
}

impl Optolink {
  pub(crate) const TIMEOUT: Duration = Duration::from_secs(60);

  /// Opens a serial device.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use vcontrol::Optolink;
  ///
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// let mut device = Optolink::open("/dev/ttyUSB0")?;
  /// # Ok(())
  /// # }
  /// ```
  pub async fn open(port: impl AsRef<str>) -> io::Result<Optolink> {
    log::trace!("Optolink::open(…)");

    let serial_port = tokio_serial::new(port.as_ref(), 4800)
      .data_bits(DataBits::Eight)
      .flow_control(FlowControl::None)
      .parity(Parity::Even)
      .stop_bits(StopBits::Two)
      .open_native_async();

    let serial_port = match serial_port {
      Ok(serial_port) => serial_port,
      Err(err) => return match err.kind {
        tokio_serial::ErrorKind::NoDevice => Err(io::Error::new(io::ErrorKind::NotFound, err.description)),
        tokio_serial::ErrorKind::InvalidInput => Err(io::Error::new(io::ErrorKind::InvalidInput, err.description)),
        tokio_serial::ErrorKind::Unknown => Err(io::Error::new(io::ErrorKind::Other, err.description)),
        tokio_serial::ErrorKind::Io(kind) => Err(io::Error::new(kind, err.description)),
      }
    };

    Ok(Optolink { device: Device::Tty(serial_port), timeout: None })
  }

  /// Connects to a device via TCP.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use vcontrol::Optolink;
  ///
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// let mut device = Optolink::connect(("localhost", 1234))?;
  /// # Ok(())
  /// # }
  /// ```
  pub async fn connect(addr: impl ToSocketAddrs) -> io::Result<Optolink> {
    log::trace!("Optolink::connect(…)");

    let addrs: Vec<SocketAddr> = addr.to_socket_addrs()?.collect();

    let stream = TcpStream::connect(&addrs as &[SocketAddr]).await
      .map_err(|err| {
        io::Error::new(err.kind(), format!("{}: {}", err, addrs.iter().map(|addr| addr.to_string()).collect::<Vec<String>>().join(", ")))
      })?;

    Ok(Optolink { device: Device::Stream(stream), timeout: None })
  }

  /// Purge all contents of the input buffer.
  pub async fn purge(&mut self) -> Result<(), io::Error> {
    log::trace!("Optolink::purge()");

    match &mut self.device {
      Device::Tty(tty) => {
        let mut buf = [0];

        loop {
          match tty.try_read(&mut buf) {
            Ok(_) => continue,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
            Err(e) => return Err(e),
          }
        }
      }
      Device::Stream(stream) => {
        let mut buf = [0];

        loop {
          match stream.try_read(&mut buf) {
            Ok(_) => continue,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
            Err(e) => return Err(e),
          }
        }
      },
    }

    Ok(())
  }
}

impl AsyncWrite for Optolink {
  fn poll_write(
      self: Pin<&mut Self>,
      cx: &mut Context<'_>,
      buf: &[u8]
  ) -> Poll<tokio::io::Result<usize>> {
    log::trace!("Optolink::poll_write(…)");

    let this = self.project();

    match this.device.project() {
      DeviceProj::Tty(tty) => tty.poll_write(cx, buf),
      DeviceProj::Stream(stream) => stream.poll_write(cx, buf),
    }
  }

  fn poll_flush(
      self: Pin<&mut Self>,
      cx: &mut Context<'_>
  ) -> Poll<tokio::io::Result<()>> {
    log::trace!("Optolink::poll_flush()");

    let this = self.project();

    match this.device.project() {
      DeviceProj::Tty(tty) => tty.poll_flush(cx),
      DeviceProj::Stream(stream) => stream.poll_flush(cx),
    }
  }

  fn poll_shutdown(
      self: Pin<&mut Self>,
      cx: &mut Context<'_>
  ) -> Poll<tokio::io::Result<()>> {
    log::trace!("Optolink::poll_shutdown()");

    let this = self.project();

    match this.device.project() {
      DeviceProj::Tty(tty) => tty.poll_shutdown(cx),
      DeviceProj::Stream(stream) => stream.poll_shutdown(cx),
    }
  }
}

impl AsyncRead for Optolink {
  fn poll_read(
      self: Pin<&mut Self>,
      cx: &mut Context<'_>,
      buf: &mut ReadBuf<'_>
  ) -> Poll<tokio::io::Result<()>> {
    log::trace!("Optolink::poll_read()");

    let this = self.project();

    match this.device.project() {
      DeviceProj::Tty(tty) => tty.poll_read(cx, buf),
      DeviceProj::Stream(stream) => stream.poll_read(cx, buf),
    }
  }
}
