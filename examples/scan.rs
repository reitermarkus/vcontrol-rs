use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Seek;
use std::collections::BTreeMap;

use vcontrol::{Device, Optolink, AccessMode, Command, DataType, RawType, VControl, device::VBC550P, Value};
use vcontrol::Protocol;

fn main() {
  env_logger::init();

  let optolink_port = env::args().nth(1).expect("no serial port specified");
  let mut optolink = if optolink_port.contains(':') {
    Optolink::connect(optolink_port)
  } else {
    Optolink::open(optolink_port)
  }.unwrap();


  let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open("scan.yml").unwrap();
  let cache: BTreeMap<u16, u8> = serde_yaml::from_reader(&mut file).unwrap_or_default();

  <VBC550P as Device>::Protocol::negotiate(&mut optolink).unwrap();

  for i in 0..u16::MAX {
    let previous_value = if let Some(previous_value) = cache.get(&i) {
      if *previous_value == 255 {
        continue
      }

      Some(*previous_value)
    } else {
      None
    };

    let command = Command {
      addr: i,
      mode: AccessMode::Read,
      data_type: DataType::Int,
      raw_type: RawType::U8,
      block_len: 1,
      byte_len: 1,
      byte_pos: 0,
      bit_pos: None,
      bit_len: None,
      factor: None,
      mapping: None,
    };

    loop {
      match VBC550P::get(&mut optolink, &command) {
        Ok(Value::Empty) => {
          eprintln!("0x{:04X}: {}", command.addr, 255);
          writeln!(file, "0x{:04X}: {}", command.addr, 255);
        },
        Ok(Value::Int(n)) => {
          if let Some(ref previous_value) = previous_value {
            // Don't record new value when it is the same as before.
            if n as u8 == *previous_value {
              break
            }
          }

          eprintln!("0x{:04X}: {}", command.addr, n);
          writeln!(file, "0x{:04X}: {}", command.addr, n);
        },
        Ok(_) => unreachable!(),
        Err(e) => {
          eprintln!("Error: {}", e);
          <VBC550P as Device>::Protocol::negotiate(&mut optolink).unwrap();
          continue
        }
      }

      break
    }
  }
}
