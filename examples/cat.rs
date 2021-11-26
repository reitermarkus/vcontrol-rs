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
    let readable = commands.get(key).map(|c| c.mode.is_read()).unwrap_or(false);
    if !readable {
      continue;
    }

    let res = vcontrol.get(key);

    match res {
      Ok((value, value_meta)) => {
        if !matches!(value, Value::Empty) {
          println!("{}:", key);

          match &value {
            Value::Int(n) => print!("{}", n),
            Value::Double(n) => print!("{}", n),
            Value::Array(array) => print!("{:?}", array),
            Value::DateTime(date_time) => print!("{}", date_time),
            Value::Error(error) => print!("{} - {}", error.time(), error.to_str(vcontrol.device()).unwrap()),
            Value::CircuitTimes(cycle_times) => print!("{:#?}", cycle_times),
            Value::String(string) => print!("{}", string),
            Value::Empty => (),
          }

          match value_meta {
            ValueMeta::None => (),
            ValueMeta::Unit(unit) => print!(" {}", unit),
            ValueMeta::Mapping(mapping) => if let Value::Int(value) = value {
              print!(": {}", mapping.get(&(value as i32)).unwrap());
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
