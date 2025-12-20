use std::collections::BTreeMap;

use vcontrol::{Optolink, VControl, Value};

pub async fn cat(optolink: Optolink) -> Result<(), Box<dyn std::error::Error>> {
  let mut vcontrol = VControl::connect(optolink).await?;

  log::info!("Connected to '{}' via {} protocol.", vcontrol.device().name(), vcontrol.protocol());

  let mut commands = BTreeMap::new();

  for (command_name, command) in vcontrol::commands::system_commands() {
    commands.insert(command_name, command);
  }

  for (command_name, command) in vcontrol.device().commands() {
    commands.insert(command_name, command);
  }

  for (command_name, command) in commands {
    let readable = command.access_mode().is_read();
    if !readable {
      log::warn!("Command '{command_name}' is not readable.");
      continue;
    }

    let res = vcontrol.get(command_name).await;

    match res {
      Ok(value) => {
        println!("{}:", command_name);
        if matches!(value.value, Value::Empty) {
          println!("<empty>");
        } else {
          println!("{}", value);
        }
      },
      Err(err) => {
        eprintln!("{} error: {:?}", command_name, err);
      },
    }
  }

  Ok(())
}
