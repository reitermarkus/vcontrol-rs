use std::env;
use std::error::Error;

use vcontrol::{Optolink, VControl, Value, ValueMeta};

fn main() -> Result<(), Box<dyn Error + Send + Sync>>  {
  env_logger::init();

  let optolink_port = env::args().nth(1).expect("no serial port specified");
  let optolink = if optolink_port.contains(':') {
    Optolink::connect(optolink_port)
  } else {
    Optolink::open(optolink_port)
  }?;

  let mut vcontrol = VControl::connect(optolink)?;

  println!("Protocol: {:?}", vcontrol.protocol());
  println!("Device: {:?}", vcontrol.device().name());
  println!();

  let commands = vcontrol.device().commands();
  let mut keys = commands.keys().collect::<Vec<_>>();
  keys.sort();

  for key in keys {

    let res = vcontrol.get(key);

    match res {
      Ok((value, value_meta)) => {
        if !matches!(value, Value::Empty) {
          println!("{}:", key);

          print!("  {:?}", value);

          match value_meta {
            ValueMeta::None => (),
            ValueMeta::Unit(unit) => print!(" {}", unit),
            ValueMeta::Mapping(mapping) => if let Value::Int(value) = value {
              print!(": {:?}", mapping.get(&(value as i32)));
            }
          }

          println!();
        }
      },
      Err(err) => {
        eprintln!("{} error: {:?}", key, err);
      },
    }
  }

  Ok(())
}
