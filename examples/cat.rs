use std::env;
use std::error::Error;

use vcontrol::{Optolink, VControl, Value};

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
      Ok(value) => {
        if !matches!(value.value, Value::Empty) {
          println!("{}:", key);
          println!("{}", value);
        }
      },
      Err(err) => {
        eprintln!("{} error: {:?}", key, err);
      },
    }
  }

  Ok(())
}
