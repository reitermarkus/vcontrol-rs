use std::process::exit;
use std::sync::{RwLock, Weak};

use clap::{crate_version, Arg, App, SubCommand, AppSettings::ArgRequiredElseHelp};
use serde_json;
use webthing::{
  Action, Thing, ThingsType, WebThingServer,
  server::ActionGenerator,
};

use vcontrol::{Optolink, VControl, Value};

struct Generator;

impl ActionGenerator for Generator {
  fn generate(
    &self,
    _thing: Weak<RwLock<Box<dyn Thing>>>,
    _name: String,
    _input: Option<&serde_json::Value>,
  ) -> Option<Box<dyn Action>> {
    None
  }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();

  let app = App::new("vcontrol")
              .version(crate_version!())
              .setting(ArgRequiredElseHelp)
              .help_short("?")
              .arg(Arg::with_name("device")
                .short("d")
                .long("device")
                .takes_value(true)
                .conflicts_with_all(&["host", "port"])
                .help("path of the device"))
              .arg(Arg::with_name("host")
                .short("h")
                .long("host")
                .takes_value(true)
                .conflicts_with("device")
                .requires("port")
                .help("hostname or IP address of the device (default: localhost)"))
              .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .conflicts_with("device")
                .help("port of the device"))
              .subcommand(SubCommand::with_name("get")
                .about("get value")
                .arg(Arg::with_name("command")
                  .help("name of the command")
                  .required(true)))
              .subcommand(SubCommand::with_name("set")
                .about("set value")
                .arg(Arg::with_name("command")
                  .help("name of the command")
                  .required(true))
                .arg(Arg::with_name("value")
                  .help("value")
                  .required(true)))
              .subcommand(SubCommand::with_name("server")
                .about("start web server"));

  let matches = app.get_matches();

  let mut vcontrol = if let Some(device) = matches.value_of("device") {
    Optolink::open(device)
      .map(|device| VControl::connect(device).unwrap())
  } else if let Some(port) = matches.value_of("port") {
    let host = matches.value_of("host").unwrap_or("localhost");
    let port = port.parse().unwrap_or_else(|_| {
      eprintln!("Error: Could not parse port from “{}”.", port);
      exit(1);
    });

    Optolink::connect((host, port))
      .map(|device| VControl::connect(device).unwrap())
  } else {
    unreachable!()
  }.unwrap_or_else(|err| {
    eprintln!("Error: {}", err);
    exit(1);
  });

  log::info!("Connected to '{}' via {} protocol.", vcontrol.device().name(), vcontrol.protocol());

  if let Some(matches) = matches.subcommand_matches("get") {
    let command = matches.value_of("command").unwrap();

    match vcontrol.get(command) {
      Ok(output_value) => {
        println!("{}", serde_json::to_string_pretty(&output_value).unwrap());
      },
      Err(err) => {
        eprintln!("Error: {}", err);
        exit(1);
      }
    }
  }

  if let Some(matches) = matches.subcommand_matches("set") {
    let command = matches.value_of("command").unwrap();
    let value = matches.value_of("value").unwrap();

    let input_value: Value = serde_json::from_str(&value).unwrap_or(Value::String(value.to_string()));

    match vcontrol.set(command, input_value) {
      Ok(()) => {},
      Err(err) => {
        eprintln!("Error: {}", err);
        exit(1);
      }
    }
  }

  if let Some(_matches) = matches.subcommand_matches("server") {
    let port = 8888;

    let mut server = WebThingServer::new(
      ThingsType::Single(vcontrol.into_thing()),
      Some(port),
      None,
      None,
      Box::new(Generator),
      None,
      Some(true),
    );

    server.start(None).await?;
  }

  Ok(())
}
