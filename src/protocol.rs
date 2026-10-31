use std::{fmt, io};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::Optolink;

mod vs1;
use self::vs1::Vs1;

mod vs2;
use self::vs2::Vs2;

pub(crate) trait ProtocolTransport: AsyncRead + AsyncWrite + Unpin {
  async fn purge(&mut self) -> Result<(), io::Error>;
}

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
  Vs1,
  Vs2,
}

impl Protocol {
  /// Try detecting the protocol automatically.
  pub async fn detect(o: &mut Optolink) -> Option<Self> {
    if Vs2::negotiate(o).await.is_ok() {
      return Some(Self::Vs2);
    }

    if Vs1::negotiate(o).await.is_ok() {
      return Some(Self::Vs1);
    }

    None
  }

  /// Negotiate the protocol.
  pub async fn negotiate(&self, o: &mut Optolink) -> Result<(), io::Error> {
    self.negotiate_with_transport(o).await
  }

  async fn negotiate_with_transport<T: ProtocolTransport>(&self, o: &mut T) -> Result<(), io::Error> {
    match self {
      Self::Vs1 => Vs1::negotiate(o).await,
      Self::Vs2 => Vs2::negotiate(o).await,
    }
  }

  /// Reads the value at the address `addr` into `buf`.
  pub async fn get(&self, o: &mut Optolink, addr: u16, buf: &mut [u8]) -> Result<(), io::Error> {
    self.get_with_transport(o, addr, buf).await
  }

  async fn get_with_transport<T: ProtocolTransport>(&self, o: &mut T, addr: u16, buf: &mut [u8]) -> Result<(), io::Error> {
    match self {
      Self::Vs1 => Vs1::get(o, addr, buf).await,
      Self::Vs2 => Vs2::get(o, addr, buf).await,
    }
  }

  /// Writes the given value `value` to the the address `addr`.
  pub async fn set(&self, o: &mut Optolink, addr: u16, value: &[u8]) -> Result<(), io::Error> {
    self.set_with_transport(o, addr, value).await
  }

  async fn set_with_transport<T: ProtocolTransport>(&self, o: &mut T, addr: u16, value: &[u8]) -> Result<(), io::Error> {
    match self {
      Self::Vs1 => Vs1::set(o, addr, value).await,
      Self::Vs2 => Vs2::set(o, addr, value).await,
    }
  }
}

impl fmt::Display for Protocol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Vs1 => "VS1",
      Self::Vs2 => "VS2",
    }
    .fmt(f)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream, duplex};

  impl ProtocolTransport for DuplexStream {
    async fn purge(&mut self) -> Result<(), io::Error> {
      Ok(())
    }
  }

  fn vs2_telegram(message_type: u8, function: u8, addr: u16, payload_len: u8, payload: Option<&[u8]>) -> Vec<u8> {
    let [addr_high, addr_low] = addr.to_be_bytes();
    let payload_bytes = payload.unwrap_or_default();
    let mut telegram = vec![0x41, 5 + payload_bytes.len() as u8, message_type, function, addr_high, addr_low, payload_len];
    telegram.extend_from_slice(payload_bytes);
    let checksum = telegram[1..].iter().fold(0u8, |sum, byte| sum.wrapping_add(*byte));
    telegram.push(checksum);
    telegram
  }

  #[tokio::test]
  async fn vs1_get_runs_over_mock_transport() {
    let (mut client, mut peer) = duplex(64);
    let peer_task = tokio::spawn(async move {
      let mut reset = [0; 1];
      peer.read_exact(&mut reset).await.unwrap();
      assert_eq!(reset, [0x04]);
      peer.write_all(&[0x05]).await.unwrap();

      let mut request = [0; 5];
      peer.read_exact(&mut request).await.unwrap();
      assert_eq!(request, [0x01, 247, 0x12, 0x34, 1]);
      peer.write_all(&[0xA5]).await.unwrap();
    });

    let mut value = [0];
    Protocol::Vs1.get_with_transport(&mut client, 0x1234, &mut value).await.unwrap();
    assert_eq!(value, [0xA5]);
    peer_task.await.unwrap();
  }

  #[tokio::test]
  async fn vs2_get_runs_over_mock_transport() {
    let (mut client, mut peer) = duplex(64);
    let peer_task = tokio::spawn(async move {
      let mut request = [0; 8];
      peer.read_exact(&mut request).await.unwrap();
      assert_eq!(request, vs2_telegram(0, 1, 0x1234, 2, None).as_slice());
      peer.write_all(&[0x06]).await.unwrap();

      peer.write_all(&vs2_telegram(1, 1, 0x1234, 2, Some(&[0xA5, 0x5A]))).await.unwrap();
      let mut ack = [0; 1];
      peer.read_exact(&mut ack).await.unwrap();
      assert_eq!(ack, [0x06]);
    });

    let mut value = [0; 2];
    Protocol::Vs2.get_with_transport(&mut client, 0x1234, &mut value).await.unwrap();
    assert_eq!(value, [0xA5, 0x5A]);
    peer_task.await.unwrap();
  }

  #[tokio::test]
  async fn vs1_set_runs_over_mock_transport() {
    let (mut client, mut peer) = duplex(64);
    let peer_task = tokio::spawn(async move {
      let mut reset = [0; 1];
      peer.read_exact(&mut reset).await.unwrap();
      assert_eq!(reset, [0x04]);
      peer.write_all(&[0x05]).await.unwrap();

      let mut request = [0; 7];
      peer.read_exact(&mut request).await.unwrap();
      assert_eq!(request, [0x01, 244, 0x12, 0x34, 2, 0xA5, 0x5A]);
      peer.write_all(&[0x00]).await.unwrap();
    });

    Protocol::Vs1.set_with_transport(&mut client, 0x1234, &[0xA5, 0x5A]).await.unwrap();
    peer_task.await.unwrap();
  }

  #[tokio::test]
  async fn vs2_set_runs_over_mock_transport() {
    let (mut client, mut peer) = duplex(64);
    let peer_task = tokio::spawn(async move {
      let request = vs2_telegram(0, 2, 0x1234, 2, Some(&[0xA5, 0x5A]));
      let mut received_request = vec![0; request.len()];
      peer.read_exact(&mut received_request).await.unwrap();
      assert_eq!(received_request, request);
      peer.write_all(&[0x06]).await.unwrap();

      peer.write_all(&vs2_telegram(1, 2, 0x1234, 2, None)).await.unwrap();
      let mut ack = [0; 1];
      peer.read_exact(&mut ack).await.unwrap();
      assert_eq!(ack, [0x06]);
    });

    Protocol::Vs2.set_with_transport(&mut client, 0x1234, &[0xA5, 0x5A]).await.unwrap();
    peer_task.await.unwrap();
  }
}
