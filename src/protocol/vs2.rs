use core::fmt;
use std::io;

use num_enum::TryFromPrimitive;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::Optolink;

const LEADIN: [u8; 1] = [0x41];

const START: [u8; 3] = [0x16, 0x00, 0x00];
const RESET: [u8; 1] = [0x04];
const SYNC:  [u8; 1] = [0x05];
const ACK:   [u8; 1] = [0x06];
const NACK:  [u8; 1] = [0x15];

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum MessageType {
  Request = 0,
  Response = 1,
  Unacknowledged = 2,
  Error = 3,
}

impl fmt::Display for MessageType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Request => "request",
      Self::Response => "response",
      Self::Unacknowledged => "unacknowledged",
      Self::Error => "error",
    }.fmt(f)
  }
}

struct Header {
  message_type: MessageType,
  function: Function,
  addr: u16,
  payload_len: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[allow(unused)]
#[non_exhaustive]
#[repr(u8)]
enum Function {
  VirtualRead = 1,
  VirtualWrite = 2,
  PhysicalRead = 3,
  PhysicalWrite = 4,
  EepromRead = 5,
  EepromWrite = 6,
  RemoteProcedureCall = 7,
  VirtualMbus = 33,
  VirtualMarketManagerRead = 34,
  VirtualMarketManagerWrite = 35,
  VirtualWiloRead = 36,
  VirtualWiloWrite = 37,
  XramRead = 49,
  XramWrite = 50,
  PortRead = 51,
  PortWrite = 52,
  BeRead = 53,
  BeWrite = 54,
  KmbusRamRead = 65,
  KmbusEepromRead = 67,
  KbusDataelementRead = 81,
  KbusDataelementWrite = 82,
  KbusDatablockRead = 83,
  KbusDatablockWrite = 84,
  KbusTransparentRead = 85,
  KbusTransparentWrite = 86,
  KbusInitializationRead = 87,
  KbusInitializationWrite = 88,
  KbusEepromLtRead = 89,
  KbusEepromLtWrite = 90,
  KbusControlWrite = 91,
  KbusMemberlistRead = 93,
  KbusMemberlistWrite = 94,
  KbusVirtualRead = 95,
  KbusVirtualWrite = 96,
  KbusDirectRead = 97,
  KbusDirectWrite = 98,
  KbusIndirectRead = 99,
  KbusIndirectWrite = 100,
  KbusGatewayRead = 101,
  KbusGatewayWrite = 102,
  ProcessWrite    = 120,
  ProcessRead     = 123,
  OtPhysicalRead  = 180,
  OtVirtualRead   = 181,
  OtPhysicalWrite = 182,
  OtVirtualWrite  = 183,
  GfaRead  = 201,
  GfaWrite = 202
}

#[derive(Debug)]
pub enum Vs2 {}

fn wrapping_sum<'a>(iter: impl IntoIterator<Item = &'a u8>) -> u8 {
  iter.into_iter().fold(0, |acc, &x| acc.wrapping_add(x))
}

impl Vs2 {
  async fn write_telegram(o: &mut Optolink, header: &Header, payload: Option<&[u8]>) -> Result<(), std::io::Error> {
    log::trace!("Vs2::write_telegram(…)");

    let message_type = header.message_type as u8;
    let function = header.function as u8;
    let addr = header.addr.to_be_bytes();

    let message_len = 5 + payload.map(|p| p.len() as u8).unwrap_or(0);
    let checksum: u8 = message_len
      .wrapping_add(message_type)
      .wrapping_add(function)
      .wrapping_add(wrapping_sum(&addr))
      .wrapping_add(header.payload_len as u8)
      .wrapping_add(payload.map(wrapping_sum).unwrap_or(0));

    loop {
      o.write_all(&LEADIN).await?;
      o.write_all(&[message_len, message_type, function]).await?;
      o.write_all(&addr).await?;
      o.write_all(&[header.payload_len]).await?;

      if let Some(payload) = payload {
        o.write_all(payload).await?;
      }

      o.write_all(&[checksum]).await?;
      o.flush().await?;

      let mut status = [0xff];
      o.read_exact(&mut status).await?;
      match status {
        ACK => return Ok(()),
        NACK => (),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "send telegram failed")),
      }
    }
  }

  async fn read_telegram(o: &mut Optolink, mut payload: Option<&mut [u8]>) -> Result<Header, std::io::Error> {
    log::trace!("Vs2::read_telegram(…)");

    let mut buf = [0xff];

    loop {
      o.read_exact(&mut buf).await?;
      if buf != LEADIN {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "telegram leadin expected"))
      }

      o.read_exact(&mut buf).await?;
      let message_len = buf[0];
      let mut checksum: u8 = message_len;

      o.read_exact(&mut buf).await?;
      let message_type = buf[0];
      checksum = checksum.wrapping_add(message_type);
      let message_type = MessageType::try_from(message_type).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "unknown message identifier: {message_type}")
      })?;

      o.read_exact(&mut buf).await?;
      let function = buf[0];
      checksum = checksum.wrapping_add(function);
      let function = Function::try_from(function).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, format!("unknown function: {function}"))
      })?;

      let mut addr = [0, 0];
      o.read_exact(&mut addr).await?;
      checksum = checksum.wrapping_add(wrapping_sum(&addr));
      let addr = u16::from_be_bytes(addr);

      o.read_exact(&mut buf).await?;
      let payload_len = buf[0];
      checksum = checksum.wrapping_add(payload_len);

      if let Some(ref mut payload) = payload {
        if payload_len != (message_len - 5) {
          return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("message length ({0}) does not match payload length ({1}): {0} - 5 != {1}", message_len, payload_len)
          ))
        }

        if payload.len() != payload_len as usize {
          return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid payload length, expected {}, got {}", payload.len(), payload_len)
          ))
        }

        o.read_exact(*payload).await?;
        checksum = checksum.wrapping_add(wrapping_sum(&**payload));
      } else {
        if message_len != 5 {
          return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid message length, expected 5, got {}", message_len)
          ))
        }
      }

      o.read_exact(&mut buf).await?;
      if checksum == buf[0] {
        o.write_all(&ACK).await?;
        o.flush().await?;
        return Ok(Header { message_type, function, addr, payload_len })
      }

      o.write_all(&NACK).await?;
      o.flush().await?;
    }
  }

  pub async fn negotiate(o: &mut Optolink) -> Result<(), io::Error> {
    log::trace!("Vs2::negotiate(…)");

    o.write_all(&RESET).await?;
    o.flush().await?;

    let mut status = [0xff];

    loop {
      o.read_exact(&mut status).await?;
      match status {
        SYNC => {},
        _ => continue,
      }

      o.write_all(&START).await?;
      o.flush().await?;

      o.read_exact(&mut status).await?;
      match status {
        ACK => return Ok(()),
        NACK => {}
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "protocol negotiation failed")),
      }
    }
  }

  pub async fn get(o: &mut Optolink, addr: u16, buf: &mut [u8]) -> Result<(), io::Error> {
    log::trace!("Vs2::get(…)");

    let header = Header {
      message_type: MessageType::Request,
      function: Function::VirtualRead,
      addr,
      payload_len: buf.len() as u8,
    };

    Self::write_telegram(o, &header, None).await?;
    let response_header = Self::read_telegram(o, Some(buf)).await?;

    Self::check_response(&response_header, header.function, addr)?;

    let expected_len = buf.len();
    let actual_len = response_header.payload_len as usize;
    if actual_len != expected_len {
      return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected to read {expected_len}, read {actual_len}")))
    }

    Ok(())
  }

  pub async fn set(o: &mut Optolink, addr: u16, value: &[u8]) -> Result<(), io::Error> {
    log::trace!("Vs2::set(…)");

    let header = Header {
      message_type: MessageType::Request,
      function: Function::VirtualWrite,
      addr,
      payload_len: value.len() as u8,
    };

    Self::write_telegram(o, &header, Some(value)).await?;
    let response_header = Self::read_telegram(o, None).await?;

    Self::check_response(&response_header, header.function, addr)?;

    let expected_len = value.len();
    let actual_len = response_header.payload_len as usize;
    if actual_len != expected_len {
      return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected to write {expected_len}, wrote {actual_len}")))
    }

    Ok(())
  }

  fn check_response(header: &Header, function: Function, addr: u16) -> Result<(), io::Error> {
    if header.message_type != MessageType::Response {
      return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected response message identifier, got {}", header.message_type)))
    }

    if header.function != function {
      return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected function {:?}, got {:?}", function, header.function)))
    }

    if header.addr != addr {
      return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected address {}, got {}", addr, header.addr)))
    }

    Ok(())
  }
}
