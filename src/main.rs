use std::{process::exit, sync::Arc};

use clap::{Arg, ArgAction, Command, crate_version};
use serde_json;
use webthing::{BaseActionGenerator, ThingsType, WebThingServer};

use vcontrol::{Optolink, VControl, Value};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();

  let app = Command::new("vcontrol")
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
    .subcommand(Command::new("server").about("start web server"));

  let matches = app.get_matches();

  let mut vcontrol = if let Some(device) = matches.get_one::<String>("device") {
    match Optolink::open(device).await {
      Ok(device) => VControl::connect(device).await,
      Err(err) => Err(err.into()),
    }
  } else if let Some(port) = matches.get_one::<String>("port") {
    let host = matches.get_one::<String>("host").map_or("localhost", |host| host);
    let port = port.parse().unwrap_or_else(|_| {
      eprintln!("Error: Could not parse port from “{}”.", port);
      exit(1);
    });

    match Optolink::connect((host, port)).await {
      Ok(device) => VControl::connect(device).await,
      Err(err) => Err(err.into()),
    }
  } else {
    unreachable!()
  }
  .unwrap_or_else(|err| {
    eprintln!("Error: {}", err);
    exit(1);
  });

  log::info!("Connected to '{}' via {} protocol.", vcontrol.device().name(), vcontrol.protocol());

  if let Some(matches) = matches.subcommand_matches("get") {
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
  }

  if let Some(matches) = matches.subcommand_matches("set") {
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
  }

  if let Some(_matches) = matches.subcommand_matches("server") {
    let port = 8888;

    let (vcontrol, thing, commands) = vcontrol::thing::make_thing(vcontrol);
    let weak_thing = Arc::downgrade(&thing);

    let mut server = WebThingServer::new(
      ThingsType::Single(thing),
      Some(port),
      None,
      None,
      Box::new(BaseActionGenerator),
      None,
      Some(true),
    );

    let server_thread = server.start(None);
    let update_thread = vcontrol::thing::update_thread(vcontrol, weak_thing, commands);

    let (server, _) = tokio::join!(server_thread, update_thread);
    server?;
  }

  Ok(())
}
