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

impl Vs2 {
  async fn write_telegram(o: &mut Optolink, addr: u16, function: Function, value_len: usize, value: &[u8]) -> Result<(), std::io::Error> {
    log::trace!("Vs2::write_telegram(…)");

    let message_type = MessageType::Request as u8;
    let function = function as u8;
    let addr = addr.to_be_bytes();

    let message_length = 5 + value.len() as u8;
    let checksum: u8 = message_length
      .wrapping_add(message_type)
      .wrapping_add(function)
      .wrapping_add(addr.iter().fold(0, |acc, &x| acc.wrapping_add(x)))
      .wrapping_add(value_len as u8)
      .wrapping_add(value.iter().fold(0, |acc, &x| acc.wrapping_add(x)));

    loop {
      o.write_all(&LEADIN).await?;
      o.write_all(&[message_length, message_type, function]).await?;
      o.write_all(&addr).await?;
      o.write_all(&[value_len as u8]).await?;
      o.write_all(value).await?;
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

  async fn read_telegram(o: &mut Optolink) -> Result<Vec<u8>, std::io::Error> {
    log::trace!("Vs2::read_telegram(…)");

    let mut buf = [0xff];

    loop {
      o.read_exact(&mut buf).await?;
      if buf != LEADIN {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "telegram leadin expected"))
      }

      o.read_exact(&mut buf).await?;
      let message_length = buf[0];

      let mut message = vec![0; message_length as usize];
      o.read_exact(&mut message).await?;
      let message = message;

      let checksum: u8 = message.iter().fold(message_length, |acc, &x| acc.wrapping_add(x));

      o.read_exact(&mut buf).await?;
      if checksum == buf[0] {
        o.write_all(&ACK).await?;
        o.flush().await?;
        return Ok(message)
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

    let function = Function::VirtualRead;

    Self::write_telegram(o, addr, function, buf.len(), &[]).await?;
    let response = Self::read_telegram(o).await?;

    let response = Self::check_response(&response, function, addr)?;

    let expected_len = buf.len();
    let actual_len = response[0] as usize;
    if actual_len != expected_len {
      return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected to read {expected_len}, read {actual_len}")))
    }

    buf.clone_from_slice(&response[1..(1 + expected_len)]);

    Ok(())
  }

  pub async fn set(o: &mut Optolink, addr: u16, value: &[u8]) -> Result<(), io::Error> {
    log::trace!("Vs2::set(…)");

    let function = Function::VirtualWrite;

    Self::write_telegram(o, addr, function, value.len(), value).await?;
    let response = Self::read_telegram(o).await?;

    let response = Self::check_response(&response, function, addr)?;

    let expected_len = value.len();
    let actual_len = response[0] as usize;
    if actual_len != expected_len {
      return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected to write {expected_len}, wrote {actual_len}")))
    }

    Ok(())
  }

  fn check_response(bytes: &[u8], expected_function: Function, addr: u16) -> Result<&[u8], io::Error> {
    if let Some(response) = bytes.get(0) {
      match MessageType::try_from(*response) {
        Ok(MessageType::Response) => (),
        Ok(message_type) => {
          return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected response message identifier, got {message_type}")))
        },
        _ => {
          return Err(io::Error::new(io::ErrorKind::InvalidData, "unknown message identifier: {response}"))
        }
      }
    } else {
      return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "expected response message identifier"))
    }

    if let Some(function) = bytes.get(1) {
      match Function::try_from(*function) {
        Ok(function) if function == expected_function => (),
        Ok(function) => {
          return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected function {expected_function:?}, got {function:?}")))
        }
        _ => {
          return Err(io::Error::new(io::ErrorKind::InvalidData, format!("unknown function: {function:?}")))
        }
      }
    } else {
      return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "expected function"))
    }

    if let Some((a1, a2)) = bytes.get(2).zip(bytes.get(3)) {
      let actual_addr = u16::from_be_bytes([*a1, *a2]);
      if actual_addr != addr {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected address {actual_addr}, {addr}")))
      }
    } else {
      return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "expected address"))
    }

    Ok(&bytes[4..])
  }
}
