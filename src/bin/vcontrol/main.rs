use std::{collections::HashMap, process::exit};

use clap::{Arg, ArgAction, Command, crate_version};
use serde_json;

use vcontrol::{Optolink, VControl, Value};

mod scan;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();

  let app = Command::new("vcontrol")
    .disable_help_flag(true)
    .version(crate_version!())
    .arg_required_else_help(true)
    .arg(
      Arg::new("device")
        .short('d')
        .long("device")
        .action(ArgAction::Set)
        .conflicts_with_all(&["host", "port"])
        .help("path of the device"),
    )
    .arg(
      Arg::new("host")
        .short('h')
        .long("host")
        .action(ArgAction::Set)
        .conflicts_with("device")
        .requires("port")
        .help("hostname or IP address of the device (default: localhost)"),
    )
    .arg(
      Arg::new("port")
        .short('p')
        .long("port")
        .action(ArgAction::Set)
        .conflicts_with("device")
        .help("port of the device"),
    )
    .subcommand(
      Command::new("get").about("get value").arg(Arg::new("command").help("name of the command").required(true)),
    )
    .subcommand(
      Command::new("set")
        .about("set value")
        .arg(Arg::new("command").help("name of the command").required(true))
        .arg(Arg::new("value").help("value").required(true)),
    )
    .subcommand(Command::new("cat").about("get all values"))
    .subcommand(Command::new("scan").about("scan all values"));

  let matches = app.get_matches();

  let optolink = if let Some(device) = matches.get_one::<String>("device") {
    Optolink::open(device).await
  } else if let Some(port) = matches.get_one::<String>("port") {
    let host = matches.get_one::<String>("host").map_or("localhost", |host| host);
    let port = port.parse().unwrap_or_else(|_| {
      eprintln!("Error: Could not parse port from “{}”.", port);
      exit(1);
    });

    Optolink::connect((host, port)).await
  } else {
    unreachable!()
  }
  .unwrap_or_else(|err| {
    eprintln!("Error: {}", err);
    exit(1);
  });

  if let Some(matches) = matches.subcommand_matches("get") {
    let mut vcontrol = VControl::connect(optolink).await.unwrap_or_else(|err| {
      eprintln!("Error: {}", err);
      exit(1);
    });

    log::info!("Connected to '{}' via {} protocol.", vcontrol.device().name(), vcontrol.protocol());

    let command = matches.get_one::<String>("command").unwrap();

    match vcontrol.get(command).await {
      Ok(output_value) => {
        println!("{}", serde_json::to_string_pretty(&output_value).unwrap());
      },
      Err(err) => {
        eprintln!("Error: {}", err);
        exit(1);
      },
    }

    return Ok(());
  }

  if let Some(matches) = matches.subcommand_matches("set") {
    let mut vcontrol = VControl::connect(optolink).await.unwrap_or_else(|err| {
      eprintln!("Error: {}", err);
      exit(1);
    });

    log::info!("Connected to '{}' via {} protocol.", vcontrol.device().name(), vcontrol.protocol());

    let command = matches.get_one::<String>("command").unwrap();
    let value = matches.get_one::<String>("value").unwrap();

    let input_value: Value = serde_json::from_str(value).unwrap_or(Value::String(value.clone()));

    match vcontrol.set(command, input_value).await {
      Ok(()) => {},
      Err(err) => {
        eprintln!("Error: {}", err);
        exit(1);
      },
    }

    return Ok(());
  }

  if let Some(_) = matches.subcommand_matches("cat") {
    let mut vcontrol = VControl::connect(optolink).await.unwrap_or_else(|err| {
      eprintln!("Error: {}", err);
      exit(1);
    });

    log::info!("Connected to '{}' via {} protocol.", vcontrol.device().name(), vcontrol.protocol());

    let mut commands = HashMap::new();

    for (command_name, command) in vcontrol::commands::system_commands() {
      commands.insert(command_name, command);
    }

    for (command_name, command) in vcontrol.device().commands() {
      commands.insert(command_name, command);
    }

    let mut keys = commands.keys().collect::<Vec<_>>();
    keys.sort();

    for key in keys {
      let readable = commands.get(key).map(|c| c.access_mode().is_read()).unwrap_or(false);
      if !readable {
        continue;
      }

      let res = vcontrol.get(key).await;

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

    return Ok(());
  }

  if let Some(_) = matches.subcommand_matches("scan") {
    scan::scan(optolink).await.unwrap();
  }

  Ok(())
}
