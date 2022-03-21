use std::io;
use std::time::Instant;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::Optolink;

const LEADIN: [u8; 1] = [0x41];

const START: [u8; 3] = [0x16, 0x00, 0x00];
const RESET: [u8; 1] = [0x04];
const SYNC:  [u8; 1] = [0x05];
const ACK:   [u8; 1] = [0x06];
const NACK:  [u8; 1] = [0x15];

const REQUEST: u8   = 0x00;
const RESPONSE: u8  = 0x01;

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
  async fn write_telegram(o: &mut Optolink, message: &[u8]) -> Result<(), std::io::Error> {
    log::trace!("Vs2::write_telegram(…)");

    let message_length = message.len() as u8;
    let checksum: u8 = message.iter().fold(message_length, |acc, &x| acc.wrapping_add(x));

    let start = Instant::now();

    loop {
      o.write_all(&LEADIN).await?;
      o.write_all(&[message_length]).await?;
      o.write_all(message).await?;
      o.write_all(&[checksum]).await?;
      o.flush().await?;

      let mut status = [0xff];
      o.read_exact(&mut status).await?;
      match status {
        ACK => return Ok(()),
        NACK => (),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "send telegram failed")),
      }

      let stop = Instant::now();

      if (stop - start) > Optolink::TIMEOUT {
        break;
      }
    }

    Err(io::Error::new(io::ErrorKind::TimedOut, "send telegram timed out"))
  }

  async fn read_telegram(o: &mut Optolink) -> Result<Vec<u8>, std::io::Error> {
    log::trace!("Vs2::read_telegram(…)");

    let mut buf = [0xff];

    let start = Instant::now();

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

      let stop = Instant::now();

      if (stop - start) > Optolink::TIMEOUT {
        break;
      }
    }

    Err(io::Error::new(io::ErrorKind::TimedOut, "send telegram timed out"))
  }

  pub async fn negotiate(o: &mut Optolink) -> Result<(), io::Error> {
    log::trace!("Vs2::negotiate(…)");

    o.write_all(&RESET).await?;
    o.flush().await?;

    let mut status = [0xff];

    let start = Instant::now();

    loop {
      let stop = Instant::now();

      if (stop - start) > Optolink::TIMEOUT {
        break;
      }

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

    Err(io::Error::new(io::ErrorKind::TimedOut, "negotiate timed out"))
  }

  pub async fn get(o: &mut Optolink, addr: u16, buf: &mut [u8]) -> Result<(), io::Error> {
    log::trace!("Vs2::get(…)");

    let addr = addr.to_be_bytes();

    let mut read_request = Vec::new();
    read_request.extend(&[REQUEST, Function::VirtualRead as u8]);
    read_request.extend(addr);
    read_request.extend(&[buf.len() as u8]);

    Self::write_telegram(o, &read_request).await?;

    let read_response = Self::read_telegram(o).await?;

    let expected_len: usize = 5 + buf.len();
    if expected_len != read_response.len() {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "unexpected response length"))
    }
    if read_response[0..2] != [RESPONSE, Function::VirtualRead as u8] {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid read data response"))
    }
    if read_response[2..4] != addr {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "wrong address"))
    }
    if read_response[4] != buf.len() as u8 {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "wrong data length"))
    }

    buf.clone_from_slice(&read_response[5..(5 + buf.len())]);

    Ok(())
  }

  pub async fn set(o: &mut Optolink, addr: u16, value: &[u8]) -> Result<(), io::Error> {
    log::trace!("Vs2::set(…)");

    let addr = addr.to_be_bytes();

    let mut write_request = Vec::new();
    write_request.extend(&[REQUEST, Function::VirtualWrite as u8]);
    write_request.extend(addr);
    write_request.extend(&[value.len() as u8]);
    write_request.extend(value);

    Self::write_telegram(o, &write_request).await?;

    let write_response = Self::read_telegram(o).await?;

    let expected_len: usize = 5;
    if expected_len != write_response.len() {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "unexpected response length"))
    }
    if write_response[0..2] != [RESPONSE, Function::VirtualWrite as u8] {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid write data response"))
    }
    if write_response[2..4] != addr {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "wrong address"))
    }
    if write_response[4] != value.len() as u8 {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "could not write data"))
    }

    Ok(())
  }
}
