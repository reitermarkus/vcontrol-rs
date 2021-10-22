use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Seek;
use std::collections::BTreehMap;

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
            .open("brute.yml").unwrap();
  let cache: BTreehMap<u16, u8> = serde_yaml::from_reader(&mut file).unwrap_or_default();

  <VBC550P as Device>::Protocol::negotiate(&mut optolink).unwrap();
  let mut i: u16 = cache.keys().max().map(|n| *n + 1).unwrap_or(0);

  let mut previous_empty = false;
  loop {
    if cache.contains_key(&i) {
      i += 1;
      continue
    }

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

    let res = VBC550P::get(&mut optolink, &command);

    match res {
      Ok(Value::Empty) => {
        eprintln!("0x{:04X}: {}", command.addr, 255);
        writeln!(file, "0x{:04X}: {}", command.addr, 255);
        previous_empty = true;
      },
      Ok(Value::Int(n)) => {
        if previous_empty {
          eprintln!();
          writeln!(file, "");
        }
        eprintln!("0x{:04X}: {}", command.addr, n);
        writeln!(file, "0x{:04X}: {}", command.addr, n);
        previous_empty = false;
      },
      Ok(_) => unreachable!(),
      Err(e) => {
        eprintln!("Error: {}", e);
        <VBC550P as Device>::Protocol::negotiate(&mut optolink).unwrap();
        continue
      }
    }

    i += 1;
  }
}
