use core::fmt;
use std::io;

use num_enum::TryFromPrimitive;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{Optolink, commands::MAX_PAYLOAD_LEN};

const LEADIN: u8 = 0x41;

const START: [u8; 3] = [0x16, 0x00, 0x00];
const RESET: [u8; 1] = [0x04];
const SYNC:  u8 = 0x05;
const ACK:   u8 = 0x06;
const NACK:  u8 = 0x15;

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

#[derive(Debug, Clone, Copy)]
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

    const MAX_TELEGRAM_LEN: usize = 1 + 5 + MAX_PAYLOAD_LEN + 1;
    let mut buffer = [0; MAX_TELEGRAM_LEN];
    buffer[0] = LEADIN;
    buffer[1] = message_len;
    buffer[2] = message_type;
    buffer[3] = function;
    buffer[4] = addr[0];
    buffer[5] = addr[1];
    let checksum_index = if let Some(payload) = payload {
      // FIXME: Support longer payload/return error instead.
      let payload_len = payload.len();
      buffer[6] = payload_len.try_into().unwrap();
      buffer[7..(7 + payload_len)].copy_from_slice(payload);
      7 + payload_len
    } else {
      buffer[6] = header.payload_len;
      7
    };

    buffer[checksum_index] = wrapping_sum(&buffer[1..checksum_index]);
    let telegram_len = checksum_index + 1;

    loop {
      o.write_all(&buffer[..telegram_len]).await?;
      o.flush().await?;

      match Self::read_status(o).await? {
        ACK => return Ok(()),
        NACK => {
          Self::negotiate(o).await?;
        },
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "send telegram failed")),
      }
    }
  }

  async fn read_telegram(o: &mut Optolink, mut payload: Option<&mut [u8]>) -> Result<Header, io::Error> {
    log::trace!("Vs2::read_telegram(…)");

    const MAX_TELEGRAM_LEN: usize = 1 + 5 + MAX_PAYLOAD_LEN + 1;
    let mut buffer = [0; MAX_TELEGRAM_LEN];

    o.read_exact(&mut buffer[0..1]).await?;
    if buffer[0] != LEADIN {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "telegram leadin expected"))
    }

    o.read_exact(&mut buffer[1..2]).await?;
    let message_len = buffer[1];

    let checksum_index = 2 + message_len as usize;
    o.read_exact(&mut buffer[2..(checksum_index + 1)]).await?;

    let checksum = wrapping_sum(&buffer[1..checksum_index]);

    if checksum == buffer[checksum_index] {
      o.write_all(&[ACK]).await?;
      o.flush().await?;
    } else {
      o.write_all(&[NACK]).await?;
      o.flush().await?;
      return Err(io::Error::new(io::ErrorKind::InvalidData, format!("invalid checksum: {} != {}", checksum, buffer[checksum_index])))
    }

    let message_type = buffer[2];
    let message_type = MessageType::try_from(message_type).map_err(|_| {
      io::Error::new(io::ErrorKind::InvalidData, "unknown message identifier: {message_type}")
    })?;

    let function = buffer[3];
    let function = Function::try_from(function).map_err(|_| {
      io::Error::new(io::ErrorKind::InvalidData, format!("unknown function: {function}"))
    })?;

    let addr = u16::from_be_bytes([buffer[4], buffer[5]]);

    let payload_len = buffer[6];

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

      payload.copy_from_slice(&buffer[7..(7 + payload.len())])
    } else if message_len != 5 {
      return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("invalid message length, expected 5, got {}", message_len)
      ))
    }

    Ok(Header { message_type, function, addr, payload_len })
  }

  async fn read_status(o: &mut Optolink) -> Result<u8, io::Error> {
    log::trace!("Vs2::read_status(…)");
    let mut status = [0xff];
    o.read_exact(&mut status).await?;
    Ok(status[0])
  }

  async fn reset(o: &mut Optolink) -> Result<(), io::Error> {
    log::trace!("Vs2::reset(…)");

    o.purge().await?;
    o.write_all(&RESET).await?;
    o.flush().await
  }

  async fn wait_for_sync(o: &mut Optolink) -> Result<(), io::Error> {
    log::trace!("Vs2::wait_for_sync(…)");

    loop {
      match Self::read_status(o).await? {
        SYNC => break Ok(()),
        byte => {
          return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected SYNC (0x{SYNC:02X}), received 0x{byte:02X}")));
        },
      }
    }
  }

  pub async fn negotiate(o: &mut Optolink) -> Result<(), io::Error> {
    log::trace!("Vs2::negotiate(…)");

    loop {
      Self::reset(o).await?;

      Self::wait_for_sync(o).await?;

      o.write_all(&START).await?;
      o.flush().await?;

      match Self::read_status(o).await? {
        ACK => return Ok(()),
        NACK => continue,
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
      payload_len: buf.len().try_into().unwrap(),
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
      payload_len: value.len().try_into().unwrap(),
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
